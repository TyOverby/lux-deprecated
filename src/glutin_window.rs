use std::num::{FloatMath, Float};
use std::vec::MoveItems;
use std::collections::{HashMap, VecMap};
use gfx::{
    DrawState,
    ClearData,
    COLOR,
    Frame,
    ToSlice,
    DeviceHelper,
    Graphics,
    GlCommandBuffer,
    GlDevice
};
use render::state::BlendPreset;
use super::{
    Color,
    BasicShape,
    PrimitiveCanvas,
    Vertex,
    LuxResult,
    LuxError
};
use glutin::WindowBuilder;

use super::{LuxCanvas, LuxWindow, Transform, StackedTransform, LuxEvent, AbstractKey};
use super::keycodes::VirtualKeyCode;
use vecmath;

use super::gfx_integration;
use super::gfx_integration as gfxi;

type Mat4f = [[f32, ..4], ..4];
type BaseColor = [f32, ..4];

struct BasicFields {
    fill_color: Option<[f32, ..4]>,
    stroke_color: Option<[f32, ..4]>,
    border: Option<f32>,
    padding: Option<f32>,
    stroke_size: Option<f32>
}

pub struct Ellipse<'a, C: 'a> {
    fields: BasicFields,
    canvas: &'a mut C
}

pub struct Rectangle<'a, C: 'a> {
    fields: BasicFields,
    canvas: &'a mut C
}


impl <'a, C> BasicShape for Ellipse<'a, C> where C: LuxCanvas + 'a {
    fn fill(self) -> Ellipse<'a, C> {
        self
    }

    fn stroke(self) -> Ellipse<'a, C> {
        self
    }

    fn fill_color<K: Color>(mut self, color: K) -> Ellipse<'a, C> {
        self.fields.fill_color = Some(color.to_rgba());
        self
    }

    fn stroke_color<K: Color>(mut self, color: K) -> Ellipse<'a, C> {
        self.fields.stroke_color = Some(color.to_rgba());
        self
    }

    fn border(mut self, border_size: f32) -> Ellipse<'a, C> {
        self.fields.border = Some(border_size);
        self
    }

    fn padding(mut self, padding_size: f32) -> Ellipse<'a, C> {
        self.fields.padding = Some(padding_size);
        self
    }

    fn stroke_size(mut self, stroke_size: f32) -> Ellipse<'a, C> {
        self.fields.stroke_size = Some(stroke_size);
        self
    }
}

pub struct Window {
    // CANVAS
    glutin_window: ::glutin::Window,
    graphics: Graphics<GlDevice, GlCommandBuffer>,
    program: ::device::Handle<u32, ::device::shade::ProgramInfo>,
    ellipse_border_program: ::device::Handle<u32, ::device::shade::ProgramInfo>,
    draw_state: DrawState,
    frame: Frame,

    // RAW
    basis_matrix: Mat4f,
    matrix_stack: Vec<Mat4f>,
    color_stack: Vec<BaseColor>,

    // WINDOW
    title: String,

    // CANVAS
    stored_rect: Option<Shape>,
    stored_circle: Option<Shape>,
    stored_circle_border: Option<gfx_integration::EllipseBorderBatch>,

    // EVENT
    event_store: Vec<LuxEvent>,
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
    code_to_char: VecMap<char>
}


pub struct Shape {
    batch: gfx_integration::BasicBatch,
    color: Option<BaseColor>
}

impl Window {
    pub fn new() -> LuxResult<Window> {
        let window_builder =
            WindowBuilder::new()
            .with_title("Lux".to_string())
            .with_dimensions(600, 500)
            .with_vsync()
            .with_gl_debug_flag(true)
            .with_multisampling(8)
            .with_gl_version((3, 3))
            .with_visibility(true);

        let window = try!(window_builder.build().map_err(|e| {
            match e {
                ::glutin::OsError(s) => LuxError::WindowError(s)
            }
        }));

        unsafe { window.make_current(); }
        let mut device = GlDevice::new(|s| window.get_proc_address(s));
        let program = try!(device.link_program(
                            gfxi::VERTEX_SRC.clone(),
                            gfxi::FRAGMENT_SRC.clone())
                           .map_err(LuxError::ShaderError));
        let ellipse_border_program = try!(device.link_program(
                            gfxi::ELLIPSE_BORDER_VERTEX_SRC.clone(),
                            gfxi::ELLIPSE_BORDER_FRAGMENT_SRC.clone())
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
            ellipse_border_program:  ellipse_border_program,
            draw_state: DrawState::new()
                                  .blend(BlendPreset::Alpha)
                                  .multi_sample(),
            frame: Frame::new(width as u16, height as u16),
            matrix_stack: vec![],
            color_stack: vec![[0.0,0.0,0.0,1.0]],
            title: "Lux".to_string(),
            basis_matrix: basis,
            stored_rect: None,
            stored_circle: None,
            stored_circle_border: None,
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
        };
        Ok(window)
    }

    pub fn process_events(&mut self) {
        use glutin;
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
    }

    pub fn render(&mut self) {
        if !self.events_since_last_render {
            self.process_events();
        }
        self.graphics.end_frame();
        self.glutin_window.swap_buffers();
        self.matrix_stack.clear();

        let (wi, hi) = self.size();
        let (w, h) = (wi as f32, hi as f32);
        let (sx, sy) = (2.0 / w, -2.0 / h);
        self.basis_matrix[0][0] = sx;
        self.basis_matrix[1][1] = sy;
        self.frame = Frame::new(wi as u16, hi as u16);
        self.events_since_last_render = false;
    }

    //// Color

    pub fn current_color(&self) -> [f32, ..4] {
        let len = self.color_stack.len();
        self.color_stack[len - 1]
    }

    pub fn stamp_shape(&mut self, vertices: &[Vertex],
                       draw_type: super::PrimitiveType) -> Shape {
        let mesh = self.graphics.device.create_mesh(vertices);
        let slice = mesh.to_slice(draw_type);
        let batch: gfx_integration::BasicBatch =
            self.graphics.make_batch(&self.program, &mesh, slice, &self.draw_state).unwrap();
        Shape {
            batch: batch,
            color: None
        }
    }

    pub fn draw_shape(&mut self, shape: &Shape) {
        let mat = *self.current_matrix();
        let params = gfx_integration::Params {
            transform: mat,
            color: shape.color.unwrap_or_else(|| self.current_color())
        };

        self.graphics.draw(&shape.batch, &params, &self.frame)
    }
}

#[allow(unused_variables)]
impl LuxCanvas for Window {
    fn size(&self) -> (u32, u32) {
        self.window_size
    }

    /*
    fn draw_border_rect(&mut self, pos: (f32, f32), size: (f32, f32), border_size: f32) {
        use std::cmp::{partial_min, partial_max};
        let (px, py) = pos;
        let (w, h) = size;
        let smallest = partial_min(w,h).unwrap_or(0.0);
        let bs = partial_max(border_size, smallest).unwrap_or(0.0);
        self.draw_rect((px + bs, py + bs), (w - 2.0 * bs, h - 2.0 * bs));
        self.with_color([1.0,1.0,1.0,0.5], |window| {
            window.draw_rect((px+border_size, py),
                             (w-2.0*border_size, border_size));
            window.draw_rect((px+border_size, py+h-border_size),
                             (w-2.0*border_size, border_size));

            window.draw_rect((px, py),
                             (border_size, h));
            window.draw_rect((px+w-border_size, py),
                             (border_size, h));
        });
    } */

    /*
    fn draw_border_ellipse(&mut self, pos: (f32, f32), size: (f32, f32), border_size: f32) {
        let pos = (pos.0 + border_size, pos.1 + border_size);
        let size = (size.0 - border_size * 2.0, size.1 - border_size * 2.0);
        self.draw_ellipse(pos, size);
        self._draw_ellipse_border(pos, size, border_size);
    }*/

    fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), line_size: f32) {
        let (ax, ay) = start;
        let (bx, by) = end;
        let (dx, dy) = (bx - ax, by - ay);
        let length = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx);

        self.push_matrix();
        self.translate(ax, ay);
        self.rotate(angle);
        self.scale(length , line_size);
        self.translate(0.0, -0.5);
        // TODO: Remove hardcoded color.
        self.draw_rect((0.0,0.0), (1.0,1.0), [0.0, 1.0, 0.0,1.0]);
        self.pop_matrix();
    }

    fn draw_lines<I: Iterator<(f32, f32)>>(&mut self, mut positions: I, line_size: f32) {
        let l_mod = line_size / 2.0;
        let mut last = None;
        for p in positions {
            match last {
                Some((x1, y1)) => {
                    let (x2, y2) = p;
                    self.draw_line((x1, y1), (x2, y2), line_size);
                    // TODO: remove hardcoded color.
                    self.draw_ellipse((x1 - l_mod, y1 - l_mod), (line_size, line_size), [0.0, 1.0, 0.0, 1.0]);
                }
                None => { }
            }
            last = Some(p);
        }

        if let Some((lx, ly)) = last {
            // TODO: remove hardcoded color.
            self.draw_ellipse((lx - l_mod, ly - l_mod), (line_size, line_size), [0.0, 1.0, 0.0, 1.0]);
        }
    }

    fn draw_arc(&mut self, pos: (f32, f32), radius: f32, angle1: f32, angle2: f32) {
        unimplemented!();
    }

    fn with_color<C: Color>(&mut self, color: C, f: |&mut Window| -> ()) {
        self.color_stack.push(color.to_rgba());
        f(self);
        self.color_stack.pop();
    }

    fn with_border_color<C: Color>(&mut self, color: C, f: |&mut Window| -> ()) {
        unimplemented!();
    }

    fn draw_text(&mut self, pos: (f32, f32), text: &str) {
        unimplemented!();
    }
}

impl PrimitiveCanvas for Window {
    fn draw_rect(&mut self, pos: (f32, f32), size: (f32, f32), color: [f32, ..4]) {
        use std::intrinsics::transmute;
        if self.stored_rect.is_none() {
            let vertex_data = [
                Vertex{ pos: [1.0, 0.0], tex: [1.0, 0.0] },
                Vertex{ pos: [0.0, 0.0], tex: [0.0, 0.0] },
                Vertex{ pos: [0.0, 1.0], tex: [0.0, 1.0] },
                Vertex{ pos: [1.0, 0.0], tex: [1.0, 0.0] },
                Vertex{ pos: [0.0, 1.0], tex: [0.0, 1.0] },
                Vertex{ pos: [1.0, 1.0], tex: [1.0, 1.0] },
            ];
            let shape = self.stamp_shape(&vertex_data, super::TriangleList);
            self.stored_rect = Some(shape);
        }
        let (x, y) = pos;
        let (w, h) = size;
        self.push_matrix();
        self.translate(x, y);
        self.scale(w, h);

        {
            // This is safe because draw_shape does *not* mutate the shape itself.
            let shape = self.stored_rect.as_ref().unwrap();
            let mat = *self.current_matrix();
            let params = gfx_integration::Params {
                transform: mat,
                color: color
            };

            self.graphics.draw(&shape.batch, &params, &self.frame);
        }
        self.pop_matrix();
    }
    fn draw_ellipse(&mut self, pos: (f32, f32), size: (f32, f32), color: [f32, ..4]) {
        use std::intrinsics::transmute;
        if self.stored_circle.is_none() {
            let mut vertex_data = vec![];
            let pi = ::std::f32::consts::PI;
            let mut i = 0.0f32;
            while i < 2.0 * pi {
                let p = [i.sin(), i.cos()];
                vertex_data.push(Vertex{pos: p, tex: p});
                i += pi / 360.0;
            }
            let shape = self.stamp_shape(vertex_data.as_slice(), ::TriangleFan);
            self.stored_circle = Some(shape);
        }

        let shape = unsafe{ transmute(self.stored_circle.as_ref().unwrap()) };
        let (x, y) = pos;
        let (sx, sy) = size;
        let (sx, sy) = (sx/2.0, sy/2.0);
        self.push_matrix();
        self.translate(x+sx, y+sy);
        self.scale(sx, sy);
        self.draw_shape(shape);
        self.pop_matrix();
    }

    fn draw_ellipse_border(&mut self, pos: (f32, f32), size: (f32, f32),
                           border_size: f32, color: [f32, ..4]) {
        if self.stored_circle_border.is_none() {
            let mut vertex_data = vec![];
            let pi = ::std::f32::consts::PI;
            let mut i = 0.0f32;
            while i < 2.0 * pi {
                let p = [i.sin(), i.cos()];
                vertex_data.push(gfx_integration::EllipseBorderVertex{
                    pos: p,
                    tex: [0.0,0.0],
                    is_outer: 0.0
                });
                vertex_data.push(gfx_integration::EllipseBorderVertex{
                    pos: p,
                    tex: [0.0,0.0],
                    is_outer: 1.0
                });
                i += pi / 360.0;
            }
            let mesh = self.graphics.device.create_mesh(vertex_data.as_slice());
            let slice = mesh.to_slice(super::TriangleStrip);
            let batch: gfx_integration::EllipseBorderBatch =
                self.graphics.make_batch(&self.ellipse_border_program, &mesh, slice, &self.draw_state).unwrap();
            self.stored_circle_border = Some(batch);
        }
        let size = (size.0 / 2.0, size.1 / 2.0);
        let (x, y) = pos;
        let (sx, sy) = size;

        self.push_matrix();
        self.translate(x + sx, y + sy);
        self.scale(sx, sy);
        let mat = *self.current_matrix();
        let params = gfx_integration::EllipseBorderParams {
            transform: mat,
            width: border_size,
            ratio: [size.0, size.1],
            color: [1.0, 0.0, 0.0, 1.0] // TODO change this
        };

        {
            let batch = self.stored_circle_border.as_ref().unwrap();
            self.graphics.draw(batch, &params, &self.frame);
        }
        self.pop_matrix();
    }
}

impl LuxWindow for Window {
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
    fn events(&mut self) -> MoveItems<LuxEvent> {
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
