use std::vec::IntoIter;
use std::collections::{HashMap, VecMap};
use glutin;

use super::interactive::keycodes::VirtualKeyCode;
use super::{
    gfx_integration,
    LuxCanvas,
    Interactive,
    Event,
    LuxExtend,
    AbstractKey,
    Color,
    Colored,
    StackedColored,
    PrimitiveCanvas,
    Vertex,
    LuxResult,
    LuxError,
    Transform,
    StackedTransform
};

use gfx::{
    DrawState,
    BufferHandle,
    ClearData,
    COLOR,
    Frame,
    ToSlice,
    DeviceHelper,
    Graphics,
    Device,
    GlCommandBuffer,
    GlDevice
};

use render::state::BlendPreset;
use glutin::WindowBuilder;

use typemap::TypeMap;

use vecmath;

type Mat4f = [[f32, ..4], ..4];
type BaseColor = [f32, ..4];

struct CachedDrawCommand {
    typ: super::PrimitiveType,
    points: Vec<super::Vertex>,
    idxs: Option<Vec<u32>>,
}

pub struct Window {
    // CANVAS
    glutin_window: ::glutin::Window,
    graphics: Graphics<GlDevice, GlCommandBuffer>,
    program: ::device::Handle<u32, ::device::shade::ProgramInfo>,
    draw_state: DrawState,
    frame: Frame,

    draw_cache: Option<CachedDrawCommand>,

    // RAW
    basis_matrix: Mat4f,
    matrix_stack: Vec<Mat4f>,
    color_stack: Vec<(BaseColor, BaseColor)>,

    // WINDOW
    title: String,

    // EVENT
    event_store: Vec<Event>,
    mouse_pos: (i32, i32),
    window_pos: (i32, i32),
    window_size: (u32, u32),
    focused: bool,
    mouse_down_count: u8,
    events_since_last_render: bool,

    // KEY EVENTS
    codes_pressed: HashMap<u8, bool>,
    chars_pressed: HashMap<char, bool>,
    virtual_keys_pressed: HashMap<VirtualKeyCode, bool>,
    code_to_char: VecMap<char>,

    // EXTEND
    typemap: TypeMap
}

impl Window {
    pub fn new() -> LuxResult<Window> {
        use glutin::CreationError;
        let window_builder =
            WindowBuilder::new()
            .with_title("Lux".to_string())
            .with_dimensions(600, 500)
            .with_vsync()
            .with_gl_debug_flag(false)
            .with_gl_version((3, 2))
            .with_multisampling(8)
            .with_visibility(true);

        let window = try!(window_builder.build().map_err(|e| {
            match e {
                CreationError::OsError(s) =>
                    LuxError::WindowError(s),
                CreationError::NotSupported  =>
                    LuxError::WindowError("Window creation is not supported.".to_string())
            }
        }));

        unsafe { window.make_current(); }
        let mut device = GlDevice::new(|s| window.get_proc_address(s));
        let program = try!(device.link_program(
                            gfx_integration::VERTEX_SRC.clone(),
                            gfx_integration::FRAGMENT_SRC.clone())
                           .map_err(LuxError::ShaderError));
        let graphics = Graphics::new(device);
        let (width, height) = window.get_inner_size().unwrap_or((0, 0));
        let mut basis = vecmath::mat4_id();
        basis[1][1] = -1.0;
        basis[3][0] = -1.0;
        basis[3][1] = 1.0;
        let window = Window {
            glutin_window: window,
            graphics: graphics,
            program: program,
            draw_state: DrawState::new()
                                  .blend(BlendPreset::Alpha)
                                  .multi_sample(),
            draw_cache: None,
            frame: Frame::new(width as u16, height as u16),
            matrix_stack: vec![],
            color_stack: vec![([0.0,0.0,0.0,1.0], [0.0,0.0,0.0,1.0])],
            title: "Lux".to_string(),
            basis_matrix: basis,
            event_store: vec![],
            mouse_pos: (0, 0),
            window_pos: (0, 0),
            window_size: (width as u32, height as u32),
            focused: true,
            mouse_down_count: 0,
            events_since_last_render: false,
            codes_pressed: HashMap::new(),
            chars_pressed: HashMap::new(),
            virtual_keys_pressed: HashMap::new(),
            code_to_char: VecMap::new(),
            typemap: TypeMap::new(),
        };
        Ok(window)
    }

    pub fn process_events(&mut self) {
        use glutin::Event as glevent;
        self.events_since_last_render = true;
        fn t_mouse(button: glutin::MouseButton) -> super::MouseButton {
            match button {
                glutin::MouseButton::LeftMouseButton => super::Left,
                glutin::MouseButton::RightMouseButton => super::Right,
                glutin::MouseButton::MiddleMouseButton => super::Middle,
                glutin::MouseButton::OtherMouseButton(a) => super::OtherMouseButton(a)
            }
        }
        let mut last_char = None;
        for event in self.glutin_window.poll_events() { match event {
            glevent::MouseMoved((x, y)) => {
                self.mouse_pos = (x as i32, y as i32);
                self.event_store.push(super::MouseMoved((x as i32, y as i32)))
            }
            glevent::MouseInput(glutin::ElementState::Pressed, button) => {
                self.event_store.push(super::MouseDown(t_mouse(button)));
                self.mouse_down_count += 1;
            }
            glevent::MouseInput(glutin::ElementState::Released, button) => {
                self.event_store.push(super::MouseUp(t_mouse(button)));
                self.mouse_down_count -= 1;
            }
            glevent::Resized(w, h) => {
                self.window_size = (w as u32, h as u32);
                self.event_store.push(super::WindowResized(self.window_size));
            }
            glevent::Moved(x, y) => {
                self.window_pos = (x as i32, y as i32);
                self.event_store.push(super::WindowMoved(self.window_pos));
            }
            glevent::MouseWheel(i) => {
                self.event_store.push(super::MouseWheel(i as i32));
            }
            glevent::ReceivedCharacter(c) => {
                last_char = Some(c);
            }
            glevent::KeyboardInput(glutin::ElementState::Pressed, code, virt)  => {
                let c = virt.and_then(super::keycode_to_char)
                            .or(last_char.take())
                            .or_else(|| self.code_to_char.get(&(code as uint))
                                                         .map(|a| *a));
                self.event_store.push( super::KeyPressed(code, c, virt));

                if c.is_some() && !self.code_to_char.contains_key(&(code as uint)) {
                    self.code_to_char.insert(code as uint, c.unwrap());
                }

                self.codes_pressed.insert(code, true);
                if let Some(chr) = c {
                    self.chars_pressed.insert(chr, true);
                }
                if let Some(v_key) = virt {
                    self.virtual_keys_pressed.insert(v_key, true);
                }
            }
            glevent::KeyboardInput(glutin::ElementState::Released, code, virt) => {
                let c = virt.and_then(super::keycode_to_char)
                            .or_else(|| self.code_to_char.get(&(code as uint))
                                                         .map(|a| *a));
                self.event_store.push(super::KeyReleased(code, c, virt));
                self.codes_pressed.insert(code, false);
                if let Some(chr) = c {
                    self.chars_pressed.insert(chr, false);
                }
                if let Some(v_key) = virt {
                    self.virtual_keys_pressed.insert(v_key, false);
                }
            }
            glevent::Focused(f) => {
                self.focused = f;
            }
            _ => {}
        }}
    }

    pub fn clear<C: Color>(&mut self, color: C) {
        self.graphics.clear(
            ClearData{
                color: color.to_rgba(),
                depth: 1.0,
                stencil: 0
            },
            COLOR,
            &self.frame);
        self.prepare();
    }

    fn prepare(&mut self) {
        let (wi, hi) = self.size();
        let (w, h) = (wi as f32, hi as f32);
        let (sx, sy) = (2.0 / w, -2.0 / h);
        self.basis_matrix[0][0] = sx;
        self.basis_matrix[1][1] = sy;
        self.frame = Frame::new(wi as u16, hi as u16);
    }

    pub fn render(&mut self) {
        if !self.events_since_last_render {
            self.process_events();
        }
        self.push_draw();

        self.graphics.end_frame();
        self.glutin_window.swap_buffers();
        self.matrix_stack.clear();

        self.events_since_last_render = false;

    }

    fn push_draw(&mut self) {
        if let Some(ref cache_draw) = self.draw_cache {
            let &CachedDrawCommand{ref typ, ref points, ref idxs} = cache_draw;


            let mesh = self.graphics.device.create_mesh(points.as_slice());

            let slice = match idxs {
                &None => mesh.to_slice(*typ),
                &Some(ref idxs) =>
                    self.graphics.device
                        .create_buffer_static::<u32>(idxs.as_slice())
                        .to_slice(*typ)
            };

            let batch = self.graphics.make_batch(&self.program, &mesh, slice,
                                                 &self.draw_state).unwrap();
            let params = gfx_integration::ColorParams {
                transform: vecmath::mat4_id()
            };
            self.graphics.draw(&batch, &params, &self.frame);

            /*
            for attr in mesh.attributes.into_iter() {
                self.graphics.device.delete_buffer_raw(BufferHandle::from_raw(attr.buffer));
            }
            */
        }

        self.draw_cache = None;
    }
}

#[allow(unused_variables)]
impl LuxCanvas for Window {
    fn size(&self) -> (u32, u32) {
        self.window_size
    }

    fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), line_size: f32) {
        unimplemented!();
    }

    fn draw_lines<I: Iterator<(f32, f32)>>(&mut self, positions: I, line_size: f32) {
        unimplemented!();
    }

    fn draw_arc(&mut self, pos: (f32, f32), radius: f32,
                angle1: f32, angle2: f32, line_size: f32) {
        unimplemented!();
    }

    fn draw_text(&mut self, pos: (f32, f32), text: &str) {
        unimplemented!();
    }
}

impl PrimitiveCanvas for Window {
    fn draw_shape(&mut self,
                  n_typ: super::PrimitiveType,
                  n_points: &[super::Vertex],
                  idxs: Option<&[u32]>,
                  transform: Option<[[f32, ..4], ..4]>) {

        // Look at all this awful code for handling something that should
        // be dead simple!
        if self.draw_cache.is_some() {
            if self.draw_cache.as_ref().unwrap().typ != n_typ {
                self.push_draw();
                self.draw_cache = Some(CachedDrawCommand {
                    typ: n_typ,
                    points: vec![],
                    idxs: None
                });
            }
        } else {
            self.draw_cache = Some(CachedDrawCommand {
                typ: n_typ,
                points: vec![],
                idxs: None

            });
        }

        if let Some(idxs) = idxs {
            assert!(idxs.len() % 3 == 0,
                "The length of the indexes array must be a multiple of three.");
        }

        let transform = transform.unwrap_or(vecmath::mat4_id());
        let mat = vecmath::col_mat4_mul(*self.current_matrix(), transform);
        let draw_cache = self.draw_cache.as_mut().unwrap();

        let already_in = draw_cache.points.len() as u32;
        let adding = n_points.len() as u32;

        // Perform the global transforms here
        draw_cache.points.extend(n_points.iter().map(|&mut point| {
            let res = vecmath::col_mat4_transform(
                mat,
                [point.pos[0], point.pos[1], 0.0, 1.0]);
            point.pos = [res[0], res[1]];
            point
        }));

        // TODO: test this
        // TODO: replace most of this with 'extend' and 'map'.
        match (&mut draw_cache.idxs, idxs) {
            (&None, None) => { /* Do nothing */ }
            (&Some(ref mut v), None) => {
                for i in range(0, adding) {
                    v.push(already_in + i)
                }
            }
            (x@ &None, Some(l_idxs)) => {
                let mut v = vec![];
                for i in range(0, already_in) {
                    v.push(i);
                }
                for &i in l_idxs.iter() {
                    v.push(already_in + i);
                }
                *x = Some(v);
            }
            (&Some(ref mut v), Some(l_idxs)) => {
                for &i in l_idxs.iter() {
                    v.push(already_in + i);
                }
            }
        }
    }
}

impl Interactive for Window {
    fn is_open(&self) -> bool {
        !self.glutin_window.is_closed()
    }

    fn title(&self) -> &str {
        self.title.as_slice()
    }

    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        self.glutin_window.set_title(self.title.as_slice());
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.glutin_window.set_inner_size(width as uint, height as uint);
    }

    fn get_size(&self) -> (u32, u32) {
        self.glutin_window.get_inner_size()
            .map(|(w, h)| (w as u32, h as u32))
            .unwrap_or((0,0))
    }

    fn is_focused(&self) -> bool {
        self.focused
    }

    fn mouse_pos(&self) -> (i32, i32) {
        self.mouse_pos
    }

    fn mouse_down(&self) -> bool {
        self.mouse_down_count != 0
    }

    fn events(&mut self) -> IntoIter<Event> {
        use std::mem::replace;
        self.process_events();
        replace(&mut self.event_store, vec![]).into_iter()
    }

    fn is_key_pressed<K: AbstractKey>(&self, k: K) -> bool {
        match k.to_key() {
            (Some(code), _, _) => self.codes_pressed.get(&code).map(|x| *x),
            (_, Some(chr), _) => self.chars_pressed.get(&chr).map(|x| *x),
            (_, _, Some(key)) => self.virtual_keys_pressed.get(&key).map(|x| *x),
            (None, None, None) => None
        }.unwrap_or(false)
    }
}

impl Transform for Window {
    fn current_matrix_mut(&mut self) -> &mut [[f32, ..4], ..4] {
        let len = self.matrix_stack.len();
        if len == 0 {
            &mut self.basis_matrix
        } else {
            &mut self.matrix_stack[len - 1]
        }
    }

    fn current_matrix(&self) -> &[[f32, ..4], ..4] {
        if self.matrix_stack.len() == 0 {
            &self.basis_matrix
        } else {
            &self.matrix_stack[self.matrix_stack.len()-1]
        }
    }
}

impl StackedTransform for Window {
    fn push_matrix(&mut self) {
        let c = *self.current_matrix();
        self.matrix_stack.push(c);
    }

    fn pop_matrix(&mut self) {
        self.matrix_stack.pop();
    }
}

impl LuxExtend for Window {
    fn typemap(&self) -> &TypeMap {
        &self.typemap
    }

    fn typemap_mut(&mut self) -> &mut TypeMap {
        &mut self.typemap
    }
}

impl Colored for Window {
    fn current_fill_color(&self) -> &[f32, ..4] {
        let len = self.color_stack.len();
        &self.color_stack[len - 1].0
    }

    fn current_fill_color_mut(&mut self) -> &mut[f32, ..4] {
        let len = self.color_stack.len();
        &mut self.color_stack[len - 1].0
    }

    fn current_stroke_color(&self) -> &[f32, ..4] {
        let len = self.color_stack.len();
        &self.color_stack[len - 1].1
    }

    fn current_stroke_color_mut(&mut self) -> &mut[f32, ..4] {
        let len = self.color_stack.len();
        &mut self.color_stack[len - 1].1
    }
}

impl StackedColored for Window {
    fn push_colors(&mut self) {
        let colors = (*self.current_fill_color(), *self.current_stroke_color());
        self.color_stack.push(colors);
    }

    fn pop_colors(&mut self) {
        self.color_stack.pop();
    }
}
