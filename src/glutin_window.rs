use std::vec::IntoIter;
use std::collections::{HashMap, VecMap};
use std::rc::Rc;
use std::ops::Deref;
use std::cell::RefCell;
use std::path::Path;

use glutin;
use glium;
use image;

use super::interactive::keycodes::VirtualKeyCode;
use super::{
    FontCache,
    TextDraw,
    gfx_integration,
    Sprite,
    ImageError,
    SpriteLoader,
    FontLoad,
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
    StackedTransform
};

use glutin::WindowBuilder;

use typemap::TypeMap;

use vecmath;

macro_rules! draw_cmd {
    ($typ: path, $cons: ident, $act: ident, $frame: ident,
     $vbuf: ident, $idx: ident, $prog: ident, $uni: ident, $params: ident) => {
        if $act == $typ {
            let idx_buffer = $cons($idx);
            $frame.draw(&$vbuf, &idx_buffer, $prog, $uni, &$params).unwrap();
            return;
        }
    };
}

type Mat4f = [[f32; 4]; 4];
type BaseColor = [f32; 4];

struct CachedColorDraw {
    typ: super::PrimitiveType,
    points: Vec<super::ColorVertex>,
    idxs: Vec<u32>,
}

struct CachedTexDraw {
    typ: super::PrimitiveType,
    points: Vec<super::TexVertex>,
    texture: Rc<glium::texture::Texture2d>,
    idxs: Vec<u32>,
    color_mult: [f32; 4],
}

pub struct Window {
    // CANVAS
    display: glium::Display,
    color_program: Rc<glium::Program>,
    tex_program: Rc<glium::Program>,
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

    // FONT
    font_cache: Rc<RefCell<FontCache>>,

    // EXTEND
    typemap: TypeMap,
}

pub struct Frame {
    display: glium::Display,
    f: glium::Frame,
    color_program: Rc<glium::Program>,
    tex_program: Rc<glium::Program>,

    // Primitive Canvas
    color_draw_cache: Option<CachedColorDraw>,
    tex_draw_cache: Option<CachedTexDraw>,

    // Raw
    basis_matrix: Mat4f,
    matrix_stack: Vec<Mat4f>,
    color_stack: Vec<(BaseColor, BaseColor)>,

    font_cache: Rc<RefCell<FontCache>>,
}

impl Frame {
    fn new(display: &glium::Display,
           color_program: Rc<glium::Program>,
           tex_program: Rc<glium::Program>,
           clear_color: Option<[f32; 4]>,
           font_cache: Rc<RefCell<FontCache>>) -> Frame {
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
            color_draw_cache: None,
            tex_draw_cache: None,
            basis_matrix: basis,
            matrix_stack: vec![],
            color_stack: vec![([0.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 1.0])],
            font_cache: font_cache
        }
    }

    fn draw_textured_now(&mut self,
                typ: super::PrimitiveType,
                points: Vec<super::TexVertex>,
                idxs: Vec<u32>,
                base_mat: Option<[[f32; 4]; 4]>,
                texture: &glium::texture::Texture2d,
                color_mult: [f32; 4]) {
        use glium::index::*;
        use glium::index::PrimitiveType as Prim;
        use glium::Surface;
        use glium::LinearBlendingFactor::*;
        use glium::BlendingFunction::Addition;

        let vertex_buffer = glium::VertexBuffer::new(&self.display, points);
        let (frame, tex_program) = (&mut self.f, self.tex_program.deref());

        let uniform = gfx_integration::TexParams {
            matrix: base_mat.unwrap_or(vecmath::mat4_id()),
            texture: texture,
            color_mult: color_mult
        };

        let draw_params: glium::DrawParameters = glium::DrawParameters {
            depth_test: glium::DepthTest::Overwrite,
            depth_write: false,
            depth_range: (0.0, 1.0),
            blending_function: Some(glium::BlendingFunction::Addition{
                source: SourceAlpha,
                destination: OneMinusSourceAlpha
            }),
            line_width: None,
            dithering: true,
            backface_culling: glium::BackfaceCullingMode::CullingDisabled,
            polygon_mode: glium::PolygonMode::Fill,
            multisampling: true,
            viewport: None,
            scissor: None,
            draw_primitives: true,
            point_size: None,
        };

        draw_cmd!(Prim::Points, PointsList,
          typ, frame, vertex_buffer, idxs, tex_program, uniform, draw_params);
        draw_cmd!(Prim::LinesList, LinesList,
          typ, frame, vertex_buffer, idxs, tex_program, uniform, draw_params);
        draw_cmd!(Prim::LinesListAdjacency, LinesListAdjacency,
          typ, frame, vertex_buffer, idxs, tex_program, uniform, draw_params);
        draw_cmd!(Prim::LineStrip, LineStrip,
          typ, frame, vertex_buffer, idxs, tex_program, uniform, draw_params);
        draw_cmd!(Prim::LineStripAdjacency, LineStripAdjacency,
          typ, frame, vertex_buffer, idxs, tex_program, uniform, draw_params);
        draw_cmd!(Prim::TrianglesList, TrianglesList,
          typ, frame, vertex_buffer, idxs, tex_program, uniform, draw_params);
        draw_cmd!(Prim::TrianglesListAdjacency, TrianglesListAdjacency,
          typ, frame, vertex_buffer, idxs, tex_program, uniform, draw_params);
        draw_cmd!(Prim::TriangleStrip, TriangleStrip,
          typ, frame, vertex_buffer, idxs, tex_program, uniform, draw_params);
        draw_cmd!(Prim::TriangleStripAdjacency, TriangleStripAdjacency,
          typ, frame, vertex_buffer, idxs, tex_program, uniform, draw_params);
        draw_cmd!(Prim::TriangleFan, TriangleFan,
          typ, frame, vertex_buffer, idxs, tex_program, uniform, draw_params);
    }

    fn draw_colored_now(&mut self,
                typ: super::PrimitiveType,
                points: Vec<super::ColorVertex>,
                idxs: Vec<u32>,
                base_mat: Option<[[f32; 4]; 4]>) {
        use glium::index::*;
        use glium::index::PrimitiveType as Prim;
        use glium::Surface;
        use glium::LinearBlendingFactor::*;
        use glium::BlendingFunction::Addition;

        let vertex_buffer = glium::VertexBuffer::new(&self.display, points);
        let (frame, color_program) = (&mut self.f, self.color_program.deref());
        let uniform = gfx_integration::ColorParams {
            matrix: base_mat.unwrap_or(vecmath::mat4_id())
        };

        let draw_params: glium::DrawParameters = glium::DrawParameters {
            depth_test: glium::DepthTest::Overwrite,
            depth_write: false,
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
            draw_primitives: true,
            point_size: None
        };

        draw_cmd!(Prim::Points, PointsList,
          typ, frame, vertex_buffer, idxs, color_program, uniform, draw_params);
        draw_cmd!(Prim::LinesList, LinesList,
          typ, frame, vertex_buffer, idxs, color_program, uniform, draw_params);
        draw_cmd!(Prim::LinesListAdjacency, LinesListAdjacency,
          typ, frame, vertex_buffer, idxs, color_program, uniform, draw_params);
        draw_cmd!(Prim::LineStrip, LineStrip,
          typ, frame, vertex_buffer, idxs, color_program, uniform, draw_params);
        draw_cmd!(Prim::LineStripAdjacency, LineStripAdjacency,
          typ, frame, vertex_buffer, idxs, color_program, uniform, draw_params);
        draw_cmd!(Prim::TrianglesList, TrianglesList,
          typ, frame, vertex_buffer, idxs, color_program, uniform, draw_params);
        draw_cmd!(Prim::TrianglesListAdjacency, TrianglesListAdjacency,
          typ, frame, vertex_buffer, idxs, color_program, uniform, draw_params);
        draw_cmd!(Prim::TriangleStrip, TriangleStrip,
          typ, frame, vertex_buffer, idxs, color_program, uniform, draw_params);
        draw_cmd!(Prim::TriangleStripAdjacency, TriangleStripAdjacency,
          typ, frame, vertex_buffer, idxs, color_program, uniform, draw_params);
        draw_cmd!(Prim::TriangleFan, TriangleFan,
          typ, frame, vertex_buffer, idxs, color_program, uniform, draw_params);
    }
}

#[unsafe_destructor]
impl Drop for Frame {
    fn drop(&mut self) {
        self.flush_draw();
    }
}

impl Window {
    pub fn assert_no_error(&self)  {
        self.display.assert_no_error();
    }
    pub fn new() -> LuxResult<Window> {
        use glium::DisplayBuild;

        let window_builder =
            WindowBuilder::new()
            .with_title("Lux".to_string())
            .with_dimensions(600, 500)
            //.with_gl_version((3, 2))
            .with_vsync()
            .with_gl_debug_flag(false)
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

        //display.assert_no_error();

        let color_program = try!(
            glium::Program::from_source(
                 &display,
                 gfx_integration::COLOR_VERTEX_SRC,
                 gfx_integration::COLOR_FRAGMENT_SRC,
                 None));

        let tex_program = try!(
            glium::Program::from_source(
                 &display,
                 gfx_integration::TEX_VERTEX_SRC,
                 gfx_integration::TEX_FRAGMENT_SRC,
                 None));

        let (width, height): (u32, u32) = display.get_framebuffer_dimensions();

        let mut window = Window {
            display: display,
            color_program: Rc::new(color_program),
            tex_program: Rc::new(tex_program),
            closed: false,
            title: "Lux".to_string(),
            event_store: vec![],
            mouse_pos: (0, 0),
            window_pos: (0, 0),
            window_size: (width, height),
            focused: true,
            mouse_down_count: 0,
            events_since_last_render: false,
            codes_pressed: HashMap::new(),
            chars_pressed: HashMap::new(),
            virtual_keys_pressed: HashMap::new(),
            code_to_char: VecMap::new(),
            typemap: TypeMap::new(),
            // Safe because font_cache is set immediately after this.
            font_cache: unsafe { ::std::mem::uninitialized() }
        };

        let window_c = window.display.clone();
        window.font_cache = Rc::new(RefCell::new(try!(FontCache::new(|img: image::DynamicImage| {
            let img = img.flipv();
            let img = glium::texture::Texture2d::new(&window_c, img);
            Sprite::new(Rc::new(img))
        }))));

        Ok(window)
    }

    pub fn process_events(&mut self) {
        use glutin::Event as glevent;
        self.events_since_last_render = true;
        fn t_mouse(button: glutin::MouseButton) -> super::MouseButton {
            match button {
                glutin::MouseButton::Left=> super::Left,
                glutin::MouseButton::Right=> super::Right,
                glutin::MouseButton::Middle=> super::Middle,
                glutin::MouseButton::Other(a) => super::OtherMouseButton(a)
            }
        }

        let mut last_char = None;
        for event in self.display.poll_events() {
            match event {
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

                // Don't underflow!
                if self.mouse_down_count != 0 {
                    self.mouse_down_count -= 1;
                }
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

    pub fn cleared_frame<C: Color>(&mut self, clear_color: C) -> Frame {
        Frame::new(&self.display,
                   self.color_program.clone(),
                   self.tex_program.clone(),
                   Some(clear_color.to_rgba()),
                   self.font_cache.clone())
    }

    pub fn frame(&mut self) -> Frame {
        Frame::new(&self.display,
                   self.color_program.clone(),
                   self.tex_program.clone(),
                   None,
                   self.font_cache.clone())
    }
}

impl SpriteLoader for Window {
    fn load_sprite(&mut self, path: &Path) -> Result<Sprite, ImageError> {
        let img = try!(image::open(path)).flipv();
        let img = glium::texture::Texture2d::new(&self.display, img);
        Ok(Sprite::new(Rc::new(img)))
    }

    fn sprite_from_pixels(&mut self, pixels: Vec<Vec<[f32; 4]>>) -> Sprite {
        let pixels: Vec<Vec<(f32, f32, f32, f32)>> = unsafe {::std::mem::transmute(pixels)};
        Sprite::new(Rc::new(glium::texture::Texture2d::new(&self.display, pixels)))
    }

    fn sprite_from_image(&mut self, img: image::DynamicImage) -> Sprite {
        let img = img.flipv();
        let img = glium::texture::Texture2d::new(&self.display, img);
        Sprite::new(Rc::new(img))
    }
}

#[allow(unused_variables)]
impl LuxCanvas for Frame {
    fn size(&self) -> (f32, f32) {
        use glium::Surface;
        let (w, h) = self.f.get_dimensions();
        (w as f32, h as f32)
    }

    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, line_size: f32) {
        unimplemented!();
    }

    fn draw_lines<I: Iterator<Item = (f32, f32)>>(&mut self, positions: I, line_size: f32) {
        unimplemented!();
    }

    fn draw_arc(&mut self, pos: (f32, f32), radius: f32,
                angle1: f32, angle2: f32, line_size: f32) {
        unimplemented!();
    }
}

impl PrimitiveCanvas for Frame {
    fn draw_shape_no_batch(&mut self,
                           n_typ: super::PrimitiveType,
                           n_points: Vec<super::ColorVertex>,
                           idxs: Option<Vec<u32>>,
                           transform: Option<[[f32; 4]; 4]>) {
        self.flush_draw();
        let idxs = idxs.unwrap_or_else(|| (0u32 .. n_points.len() as u32).collect());
        let transform = match transform {
            Some(t) => vecmath::col_mat4_mul(*self.current_matrix(), t),
            None => *self.current_matrix()
        };
        self.draw_colored_now(n_typ, n_points, idxs, Some(transform));
    }

    fn draw_tex_no_batch(&mut self,
                           n_typ: super::PrimitiveType,
                           n_points: Vec<super::TexVertex>,
                           idxs: Option<Vec<u32>>,
                           transform: Option<[[f32; 4]; 4]>,
                           texture: &glium::texture::Texture2d,
                           color_mult: Option<[f32; 4]>) {
        self.flush_draw();
        let idxs = idxs.unwrap_or_else(||
                               (0u32 .. n_points.len() as u32).collect());
        let transform = match transform {
            Some(t) => vecmath::col_mat4_mul(*self.current_matrix(), t),
            None => *self.current_matrix()
        };
        let color_mult = color_mult.unwrap_or([1.0, 1.0, 1.0, 1.0]);
        self.draw_textured_now(n_typ, n_points, idxs, Some(transform), texture, color_mult);
    }

    fn flush_draw(&mut self) {
        if let Some(CachedColorDraw{typ, points, idxs}) =
            self.color_draw_cache.take() {
                self.draw_colored_now(typ, points, idxs, None);
            }
        if let Some(CachedTexDraw{typ, points, texture, idxs, color_mult}) =
            self.tex_draw_cache.take() {
                self.draw_textured_now(typ, points, idxs, None, texture.deref(), color_mult);
            }
    }

    fn draw_tex(&mut self,
                  n_typ: super::PrimitiveType,
                  n_points: &[super::TexVertex],
                  idxs: Option<&[u32]>,
                  transform: Option<[[f32; 4]; 4]>,
                  texture: Rc<glium::texture::Texture2d>,
                  color_mult: Option<[f32; 4]>) {
        use super::PrimitiveType::{Points, LinesList, TrianglesList};
        use std::mem::transmute;

        if self.color_draw_cache.is_some() {
            self.flush_draw();
        }
        let color_mult = color_mult.unwrap_or([1.0, 1.0, 1.0, 1.0]);

        // Look at all this awful code for handling something that should
        // be dead simple!
        if self.tex_draw_cache.is_some() {
            let mut same_type;
            let mut coherant_group;
            let mut same_color_mult;
            let mut same_tex;
            {
                let draw_cache = self.tex_draw_cache.as_ref().unwrap();
                same_type = draw_cache.typ == n_typ;
                coherant_group = match n_typ {
                    Points | LinesList | TrianglesList => true,
                    _ => false
                };
                same_color_mult = draw_cache.color_mult == color_mult;

                let our_ptr: *mut () = unsafe {transmute(draw_cache.texture.deref())};
                let otr_ptr: *mut () = unsafe {transmute(texture.deref())};
                same_tex = our_ptr == otr_ptr;
            }

            if !same_type || !coherant_group || !same_color_mult || !same_tex {
                self.flush_draw();
                self.tex_draw_cache = Some(CachedTexDraw {
                    typ: n_typ,
                    points: vec![],
                    idxs: vec![],
                    texture: texture,
                    color_mult: color_mult,
                });
            }
        } else {
            self.tex_draw_cache = Some(CachedTexDraw {
                typ: n_typ,
                points: vec![],
                idxs: vec![],
                texture: texture,
                color_mult: color_mult
            });
        }

        if let Some(idxs) = idxs {
            assert!(idxs.len() % 3 == 0,
                "The length of the indexes array must be a multiple of three.");
        }

        let transform = transform.unwrap_or(vecmath::mat4_id());
        let mat = vecmath::col_mat4_mul(*self.current_matrix(), transform);
        let draw_cache = self.tex_draw_cache.as_mut().unwrap();

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
                for i in 0 .. adding {
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

    fn draw_shape(&mut self,
                  n_typ: super::PrimitiveType,
                  n_points: &[super::ColorVertex],
                  idxs: Option<&[u32]>,
                  transform: Option<[[f32; 4]; 4]>) {
        use super::PrimitiveType::{Points, LinesList, TrianglesList};

        if self.tex_draw_cache.is_some() {
            self.flush_draw();
        }

        // Look at all this awful code for handling something that should
        // be dead simple!
        if self.color_draw_cache.is_some() {
            let same_type = self.color_draw_cache.as_ref().unwrap().typ == n_typ;
            let coherant_group = match n_typ {
                Points | LinesList | TrianglesList => true,
                _ => false
            };
            if !same_type || !coherant_group {
                self.flush_draw();
                self.color_draw_cache = Some(CachedColorDraw {
                    typ: n_typ,
                    points: vec![],
                    idxs: vec![]
                });
            }
        } else {
            self.color_draw_cache = Some(CachedColorDraw {
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
        let draw_cache = self.color_draw_cache.as_mut().unwrap();

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
                for i in 0 .. adding {
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
        &self.title[..]
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

    fn mouse_pos_int(&self) -> (i32, i32) {
        self.mouse_pos
    }

    fn mouse_pos(&self) -> (f32, f32) {
        (self.mouse_pos.0 as f32, self.mouse_pos.1 as f32)
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
            (Some(code), _, _) => self.codes_pressed.get(&code).cloned(),
            (_, Some(chr), _) => self.chars_pressed.get(&chr).cloned(),
            (_, _, Some(key)) => self.virtual_keys_pressed.get(&key).cloned(),
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


impl FontLoad for Window {
    fn load_font(&mut self, name: &str, path: &Path) -> LuxResult<()> {
        let mut font_cache = self.font_cache.borrow_mut();
        font_cache.load(name, path)
    }

    fn preload_font(&mut self, name: &str, size: u32) -> LuxResult<()> {
        let window_c = self.display.clone();

        let mut font_cache = self.font_cache.borrow_mut();
        let res = font_cache.use_font(|img: image::DynamicImage| {
            let img = img.flipv();
            let img = glium::texture::Texture2d::new(&window_c, img);
            Sprite::new(Rc::new(img))
        }, name, size);
        self.display.synchronize();
        res
    }
}


impl TextDraw for Frame {
    fn draw_text(&mut self, _text: &str, _x: f32, _y: f32) -> LuxResult<()> {
        let _c =  *self.current_fill_color();
        /*
        unsafe {
            let s: *mut Frame = transmute(self);
            let mut font_cache = (*s).font_cache.borrow_mut();
            let s: &mut Frame = transmute(s);
            font_cache.draw_onto(s, text, x, y, c)
        }*/

        Ok(())
    }

    fn set_font(&mut self, name: &str, size: u32) -> LuxResult<()> {
        use std::fs::File;

        let window_c = self.display.clone();

        let mut font_cache = self.font_cache.borrow_mut();
        let res = font_cache.use_font(|img: image::DynamicImage| {
            let img = img.flipv();

            let mut out_path = File::create("out.png").unwrap();
            let _ = img.save(&mut out_path, ::image::ImageFormat::PNG).unwrap();

            let img = glium::texture::Texture2d::new(&window_c, img);
            Sprite::new(Rc::new(img))
        }, name, size);
        self.display.synchronize();
        res
    }

    fn get_font(&self) -> (String, u32) {
        let font_cache = self.font_cache.borrow();
        let current = font_cache.current.as_ref().unwrap();
        (current.name.clone(), current.size)
    }
}
