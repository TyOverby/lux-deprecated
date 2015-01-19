use std::vec::IntoIter;
use std::collections::{HashMap, VecMap};
use std::rc::Rc;
use std::ops::Deref;

use glutin;
use glium;

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
    ColorVertex,
    LuxResult,
    LuxError,
    Transform,
    StackedTransform,
    VerticesVec,
    VerticesSlice
};

use glutin::WindowBuilder;

use typemap::TypeMap;

use vecmath;

type Mat4f = [[f32; 4]; 4];
type BaseColor = [f32; 4];

struct CachedDraw {
    typ: super::PrimitiveType,
    points: Vec<super::ColorVertex>,
    idxs: Vec<u32>,
}

pub struct Window {
    // CANVAS
    display: glium::Display,
    color_program: Rc<glium::Program>,
    tex_program:   Rc<glium::Program>,
    closed: bool,

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

pub struct Frame {
    display: glium::Display,
    f: glium::Frame<'static>, // TODO: remove 'static
    color_program: Rc<glium::Program>,
    tex_program: Rc<glium::Program>,

    // Primitive Canvas
    draw_cache: Option<CachedDraw>,

    // Raw
    basis_matrix: Mat4f,
    matrix_stack: Vec<Mat4f>,
    color_stack: Vec<(BaseColor, BaseColor)>,
}

impl Frame {
    fn new( display: &glium::Display,
            color_program: Rc<glium::Program>,
            tex_program: Rc<glium::Program>,
            clear_color: Option<[f32; 4]>) -> Frame {
        use glium::Surface;

        let mut frm = display.draw();
        if let Some(clear_color) = clear_color {
            let [r,g,b,a] = clear_color;
            frm.clear_color(r,g,b,a);
        }

        let size = frm.get_dimensions();
        let (w, h) = (size.0 as f32, size.1 as f32);
        let (sx, sy) = (2.0 / w, -2.0 / h);

        let mut basis = vecmath::mat4_id();
        basis[1][1] = -1.0;
        basis[3][0] = -1.0;
        basis[3][1] = 1.0;
        basis[0][0] = sx;
        basis[1][1] = sy;

        Frame {
            display: display.clone(),
            color_program: color_program,
            tex_program: tex_program,
            f: frm,
            draw_cache: None,
            basis_matrix: basis,
            matrix_stack: vec![],
            color_stack: vec![([0.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 1.0])],
        }
    }

    fn draw_now(&mut self,
                typ: super::PrimitiveType,
                points: VerticesVec,
                idxs: Vec<u32>,
                base_mat: Option<[[f32; 4]; 4]>) {
        use glium::index_buffer::*;
        use glium::index_buffer::PrimitiveType as Prim;
        use glium::Surface;
        use glium::LinearBlendingFactor::*;
        use glium::BlendingFunction::Addition;

        let vertex_buffer = glium::VertexBuffer::new(&self.display, points);
        let (frame, color_program, tex_program) = (&mut self.f,
                                                   self.color_program.deref(),
                                                   self.tex_program.deref());
        let uniform = gfx_integration::ColorParams {
            matrix: base_mat.unwrap_or(vecmath::mat4_id())
        };

        let draw_params: glium::DrawParameters = glium::DrawParameters {
            depth_function: glium::DepthFunction::Overwrite,
            depth_range: (0.0, 1.0),
            blending_function: Some(glium::BlendingFunction::Addition{
                source: SourceAlpha,
                destination: OneMinusDestinationAlpha
            }),
            line_width: None,
            dithering: true,
            backface_culling: glium::BackfaceCullingMode::CullingDisabled,
            polygon_mode: glium::PolygonMode::Fill,
            multisampling: true,
            viewport: None,
            scissor: None,
        };

        match typ {
            Prim::Points => {
                let idx_buffer = PointsList(idxs);
                frame.draw(&vertex_buffer,
                           &idx_buffer,
                           color_program,
                           &uniform,
                           &draw_params).unwrap();
            }
            Prim::LinesList => {
                let idx_buffer = LinesList(idxs);
                frame.draw(&vertex_buffer,
                           &idx_buffer,
                           color_program,
                           &uniform,
                           &draw_params).unwrap();
            }
            Prim::LinesListAdjacency => {
                let idx_buffer = LinesListAdjacency(idxs);
                frame.draw(&vertex_buffer,
                           &idx_buffer,
                           color_program,
                           &uniform,
                           &draw_params).unwrap();
            }
            Prim::LineStrip => {
                let idx_buffer = LineStrip(idxs);
                frame.draw(&vertex_buffer,
                           &idx_buffer,
                           color_program,
                           &uniform,
                           &draw_params).unwrap();
            }
            Prim::LineStripAdjacency => {
                let idx_buffer = LineStripAdjacency(idxs);
                frame.draw(&vertex_buffer,
                           &idx_buffer,
                           color_program,
                           &uniform,
                           &draw_params).unwrap();
            }
            Prim::TrianglesList => {
                let idx_buffer = TrianglesList(idxs);
                frame.draw(&vertex_buffer,
                           &idx_buffer,
                           color_program,
                           &uniform,
                           &draw_params).unwrap();
            }
            Prim::TrianglesListAdjacency => {
                let idx_buffer = TrianglesListAdjacency(idxs);
                frame.draw(&vertex_buffer,
                           &idx_buffer,
                           color_program,
                           &uniform,
                           &draw_params).unwrap();
            }
            Prim::TriangleStrip => {
                let idx_buffer = TriangleStrip(idxs);
                frame.draw(&vertex_buffer,
                           &idx_buffer,
                           color_program,
                           &uniform,
                           &draw_params).unwrap();
            }
            Prim::TriangleStripAdjacency => {
                let idx_buffer = TriangleStripAdjacency(idxs);
                frame.draw(&vertex_buffer,
                           &idx_buffer,
                           color_program,
                           &uniform,
                           &draw_params).unwrap();
            }
            Prim::TriangleFan => {
                let idx_buffer = TriangleFan(idxs);
                frame.draw(&vertex_buffer,
                           &idx_buffer,
                           color_program,
                           &uniform,
                           &draw_params).unwrap();
            }

            Prim::Patches{..} => {
                panic!("patches are undefined");
            }
        }
    }
}

#[unsafe_destructor]
impl  Drop for Frame {
    fn drop(&mut self) {
        self.flush_draw();
    }
}

impl Window {
    pub fn new() -> LuxResult<Window> {
        use glium::DisplayBuild;

        let window_builder =
            WindowBuilder::new()
            .with_title("Lux".to_string())
            .with_dimensions(600, 500)
            .with_vsync()
            .with_gl_debug_flag(false)
            .with_gl_version((3, 2))
            .with_multisampling(8)
            .with_visibility(true);

        let display = try!(window_builder.build_glium().map_err(|e| {
            match e {
                glium::GliumCreationError::GlutinCreationError(
                    glutin::CreationError::OsError(s)) =>
                        LuxError::WindowError(s),
                glium::GliumCreationError::GlutinCreationError(
                    glutin::CreationError::NotSupported)  =>
                        LuxError::WindowError("Window creation is not supported.".to_string()),
                glium::GliumCreationError::IncompatibleOpenGl(m) =>
                    LuxError::OpenGlError(m)
            }
        }));

        let color_program = try!(glium::Program::from_source(
             &display, gfx_integration::COLOR_VERTEX_SRC,
             gfx_integration::COLOR_FRAGMENT_SRC, None)
                                 .map_err(LuxError::ShaderError));
        let tex_program = try!(glium::Program::from_source(
             &display, gfx_integration::TEX_VERTEX_SRC,
             gfx_integration::TEX_FRAGMENT_SRC, None)
                                 .map_err(LuxError::ShaderError));

        let (width, height) = display.get_framebuffer_dimensions();

        let window = Window {
            display: display,
            color_program: Rc::new(color_program),
            tex_program: Rc::new(tex_program),
            closed: false,
            title: "Lux".to_string(),
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
        for event in self.display.poll_events().into_iter() { match event {
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
                            .or_else(|| self.code_to_char.get(&(code as usize))
                                                         .map(|a| *a));
                self.event_store.push( super::KeyPressed(code, c, virt));

                if c.is_some() && !self.code_to_char.contains_key(&(code as usize)) {
                    self.code_to_char.insert(code as usize, c.unwrap());
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
                            .or_else(|| self.code_to_char.get(&(code as usize))
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
            glevent::Closed => {
                self.closed = true;
            }
            glevent::Awakened => {  }
        }}
    }

    pub fn cleared_frame<C: Color>(&self, clear_color: C) -> Frame {
        Frame::new(&self.display,
                   self.color_program.clone(),
                   self.tex_program.clone(),
                   Some(clear_color.to_rgba()))
    }

    pub fn frame(&self) -> Frame {
        Frame::new(&self.display,
                   self.color_program.clone(),
                   self.tex_program.clone(),
                   None)
    }
}

#[allow(unused_variables)]
impl LuxCanvas for Frame {
    fn size(&self) -> (u32, u32) {
        use glium::Surface;
        let (w, h) = self.f.get_dimensions();
        (w as u32, h as u32)
    }

    fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), line_size: f32) {
        unimplemented!();
    }

    fn draw_lines<I: Iterator<Item = (f32, f32)>>(&mut self, positions: I, line_size: f32) {
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

impl PrimitiveCanvas for Frame {
    fn draw_shape_no_batch(&mut self,
                           n_typ: super::PrimitiveType,
                           n_points: VerticesVec,
                           idxs: Option<Vec<u32>>,
                           transform: Option<[[f32; 4]; 4]>) {
        self.flush_draw();
        let idxs = idxs.unwrap_or_else(||
                               range(0u32, n_points.len() as u32).collect());
        let transform = match transform {
            Some(t) => vecmath::col_mat4_mul(*self.current_matrix(), t),
            None => *self.current_matrix()
        };
        self.draw_now(n_typ, n_points, idxs, Some(transform));
    }

    fn flush_draw(&mut self) {
        if let Some(CachedDraw{typ, points, idxs}) = self.draw_cache.take() {
            self.draw_now(typ, points, idxs, None);
        }
    }

    fn draw_shape(&mut self,
                  n_typ: super::PrimitiveType,
                  n_points: VerticesSlice,
                  idxs: Option<&[u32]>,
                  transform: Option<[[f32; 4]; 4]>) {
        use super::PrimitiveType::{Points, LinesList, TrianglesList};
        // Look at all this awful code for handling something that should
        // be dead simple!
        if self.draw_cache.is_some() {
            let same_type = self.draw_cache.as_ref().unwrap().typ == n_typ;
            let coherant_group = match n_typ {
                Points | LinesList | TrianglesList => true,
                _ => false
            };
            if !same_type || !coherant_group {
                self.flush_draw();
                self.draw_cache = Some(CachedDraw {
                    typ: n_typ,
                    points: vec![],
                    idxs: vec![]
                });
            }
        } else {
            self.draw_cache = Some(CachedDraw {
                typ: n_typ,
                points: vec![],
                idxs: vec![]

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
        draw_cache.points.extend(n_points.iter().map(|&point| {
            let mut point = point.clone();
            let res = vecmath::col_mat4_transform(
                mat,
                [point.pos[0], point.pos[1], 0.0, 1.0]);
            point.pos = [res[0], res[1]];
            point
        }));

        // TODO: test this
        // TODO: replace most of this with 'extend' and 'map'.

        match idxs {
            None => {
                for i in range(0, adding) {
                    draw_cache.idxs.push(already_in + i)
                }
            }
            Some(l_idxs) => {
                for &i in l_idxs.iter() {
                    draw_cache.idxs.push(already_in + i);
                }
            }
        }
    }
}

impl Interactive for Window {
    fn is_open(&mut self) -> bool {
        self.process_events();
        !self.closed
    }

    fn was_open(&self) -> bool {
        !self.closed
    }

    fn title(&self) -> &str {
        self.title.as_slice()
    }

    fn set_title(&mut self, _title: &str) {
        // TODO: implement this somehow.  Is it possible yet?
        unimplemented!();
    }

    fn set_size(&mut self, _width: u32, _height: u32) {
        unimplemented!();
    }

    fn get_size(&self) -> (u32, u32) {
        self.window_size
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

impl Transform for Frame {
    fn current_matrix_mut(&mut self) -> &mut [[f32; 4]; 4] {
        let len = self.matrix_stack.len();
        if len == 0 {
            &mut self.basis_matrix
        } else {
            &mut self.matrix_stack[len - 1]
        }
    }

    fn current_matrix(&self) -> &[[f32; 4]; 4] {
        if self.matrix_stack.len() == 0 {
            &self.basis_matrix
        } else {
            &self.matrix_stack[self.matrix_stack.len()-1]
        }
    }
}

impl StackedTransform for Frame {
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

impl Colored for Frame {
    fn current_fill_color(&self) -> &[f32; 4] {
        let len = self.color_stack.len();
        &self.color_stack[len - 1].0
    }

    fn current_fill_color_mut(&mut self) -> &mut[f32; 4] {
        let len = self.color_stack.len();
        &mut self.color_stack[len - 1].0
    }

    fn current_stroke_color(&self) -> &[f32; 4] {
        let len = self.color_stack.len();
        &self.color_stack[len - 1].1
    }

    fn current_stroke_color_mut(&mut self) -> &mut[f32; 4] {
        let len = self.color_stack.len();
        &mut self.color_stack[len - 1].1
    }
}

impl StackedColored for Frame {
    fn push_colors(&mut self) {
        let colors = (*self.current_fill_color(), *self.current_stroke_color());
        self.color_stack.push(colors);
    }

    fn pop_colors(&mut self) {
        self.color_stack.pop();
    }
}
