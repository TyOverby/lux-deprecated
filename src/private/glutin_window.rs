use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::cell::{RefCell, RefMut};

use glutin;
use vecmath;
use glium;
use reuse_cache;

use super::interactive::keycodes::VirtualKeyCode;
use super::accessors::{
    HasDrawCache,
    HasPrograms,
    HasDisplay,
    HasFontCache,
    HasSurface,
    DrawParamMod,
    Fetch,
};

use super::interactive::{EventIterator, AbstractKey, Event, Interactive};
use super::font::FontCache;
use super::gfx_integration::{ColorVertex, TexVertex};
use super::canvas::LuxCanvas;
use super::color::Color;
use super::raw::{Colored, Transform};
use super::error::{LuxResult, LuxError};
use super::shaders::{gen_texture_shader, gen_color_shader};
use super::primitive_canvas::{
    PrimitiveCanvas,
    CachedColorDraw,
    CachedTexDraw,
    DrawParamModifier
};
use super::types::Float;

use glutin::WindowBuilder;

type Mat4f = [[f32; 4]; 4];
type BaseColor = [f32; 4];

/// A 1 to 1 correlation with a window shown on your desktop.
///
/// Lux uses Glutin for the window implementation.
pub struct Window {
    // CANVAS
    display: glium::Display,
    color_program: Rc<glium::Program>,
    tex_program: Rc<glium::Program>,
    closed: bool,

    // WINDOW
    title: String,

    // CACHES
    idx_cache: reuse_cache::ReuseCache<Vec<u16>>,
    tex_vtx_cache: reuse_cache::ReuseCache<Vec<TexVertex>>,
    color_vtx_cache: reuse_cache::ReuseCache<Vec<ColorVertex>>,

    // EVENT
    event_store: VecDeque<Event>,
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
    code_to_char: HashMap<usize, char>,

    // FONT
    font_cache: Rc<RefCell<FontCache>>,
}

/// A frame is a render target that can be drawn on.
///
/// Because frame rendering will wait on vsync, you should - as the name
/// implies - use one Frame instance per frame.
pub struct Frame {
    display: glium::Display,
    f: glium::Frame,
    color_program: Rc<glium::Program>,
    tex_program: Rc<glium::Program>,

    // Primitive Canvas
    color_draw_cache: Option<CachedColorDraw>,
    tex_draw_cache: Option<CachedTexDraw>,

    // CACHES
    idx_cache: reuse_cache::ReuseCache<Vec<u16>>,
    tex_vtx_cache: reuse_cache::ReuseCache<Vec<TexVertex>>,
    color_vtx_cache: reuse_cache::ReuseCache<Vec<ColorVertex>>,

    // Raw
    basis_matrix: Mat4f,
    color: [f32; 4],

    // Misc
    font_cache: Rc<RefCell<FontCache>>,
    draw_mod: DrawParamModifier,
}


impl Frame {
    fn new(display: &glium::Display,
           color_program: Rc<glium::Program>,
           tex_program: Rc<glium::Program>,
           idx_cache: reuse_cache::ReuseCache<Vec<u16>>,
           tex_vtx_cache: reuse_cache::ReuseCache<Vec<TexVertex>>,
           color_vtx_cache: reuse_cache::ReuseCache<Vec<ColorVertex>>,
           clear_color: Option<[f32; 4]>,
           font_cache: Rc<RefCell<FontCache>>) -> Frame {
        use glium::Surface;

        let mut frm = display.draw();
        if let Some(c) = clear_color {
            frm.clear_color(c[0],c[1],c[2],c[3]);
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
            idx_cache: idx_cache,
            tex_vtx_cache: tex_vtx_cache,
            color_vtx_cache: color_vtx_cache,
            f: frm,
            color_draw_cache: None,
            tex_draw_cache: None,
            basis_matrix: basis,
            color: [0.0, 0.0, 0.0, 1.0],
            font_cache: font_cache,
            draw_mod: DrawParamModifier::new()
        }
    }

}

impl Drop for Frame {
    fn drop(&mut self) {
        //self.display.assert_no_error();
        self.flush_draw();
    }
}

impl Window {
    /// Panics if an OpenGL error has occurred.
    pub fn assert_no_error(&self)  {
        self.display.assert_no_error(None);
    }

    /// Constructs a new window with the default settings.
    pub fn new() -> LuxResult<Window> {
        use glium::DisplayBuild;

        let window_builder =
            WindowBuilder::new()
            .with_title("Lux".to_string())
            .with_dimensions(600, 500)
            .with_vsync()
            .with_gl_debug_flag(false)
            .with_multisampling(8)
            .with_visibility(true);


        let display = try!(window_builder.build_glium().map_err(|e| {
            match e {
                glium::GliumCreationError::BackendCreationError(
                    glutin::CreationError::OsError(s)) =>
                        LuxError::WindowError(s),
                glium::GliumCreationError::BackendCreationError(
                    glutin::CreationError::NotSupported)  =>
                        LuxError::WindowError("Window creation is not supported.".to_string()),
                glium::GliumCreationError::IncompatibleOpenGl(m) =>
                    LuxError::OpenGlError(m)
            }
        }));

        let color_program = try!(gen_color_shader(&display));
        let tex_program = try!(gen_texture_shader(&display));

        let (width, height): (u32, u32) = display.get_framebuffer_dimensions();

        let font_cache = try!(FontCache::new(&display));

        let window = Window {
            display: display,
            color_program: Rc::new(color_program),
            tex_program: Rc::new(tex_program),
            closed: false,
            title: "Lux".to_string(),
            idx_cache: reuse_cache::ReuseCache::new(4, || vec![]),
            tex_vtx_cache: reuse_cache::ReuseCache::new(4, || vec![]),
            color_vtx_cache: reuse_cache::ReuseCache::new(4, || vec![]),
            event_store: VecDeque::new(),
            mouse_pos: (0, 0),
            window_pos: (0, 0),
            window_size: (width, height),
            focused: true,
            mouse_down_count: 0,
            events_since_last_render: false,
            codes_pressed: HashMap::new(),
            chars_pressed: HashMap::new(),
            virtual_keys_pressed: HashMap::new(),
            code_to_char: HashMap::new(),
            font_cache: Rc::new(RefCell::new(font_cache))
        };

        Ok(window)
    }

    // TODO: hide from docs
    /// Add the events from an iterator of events back to the internal event queue.
    pub fn restock_events<I: DoubleEndedIterator<Item=Event>>(&mut self, mut i: I) {
        while let Some(e) = i.next_back() {
            self.event_store.push_front(e);
        }
    }

    // TODO: hide from docs
    /// Query the underlying window system for events and add them to the
    /// the interal event queue.
    pub fn process_events(&mut self) {
        use glutin::Event as glevent;
        use super::interactive::*;
        use super::interactive::Event::*;
        use super::interactive::MouseButton::*;

        self.events_since_last_render = true;
        fn t_mouse(button: glutin::MouseButton) -> MouseButton {
            match button {
                glutin::MouseButton::Left=> Left,
                glutin::MouseButton::Right=> Right,
                glutin::MouseButton::Middle=> Middle,
                glutin::MouseButton::Other(a) => Other(a)
            }
        }

        let mut last_char = None;
        for event in self.display.poll_events() {
            match event {
            glevent::MouseMoved((x, y)) => {
                self.mouse_pos = (x as i32, y as i32);
                self.event_store.push_back(MouseMoved((x as i32, y as i32)))
            }
            glevent::MouseInput(glutin::ElementState::Pressed, button) => {
                self.event_store.push_back(MouseDown(t_mouse(button)));
                self.mouse_down_count += 1;
            }
            glevent::MouseInput(glutin::ElementState::Released, button) => {
                self.event_store.push_back(MouseUp(t_mouse(button)));

                // Don't underflow!
                if self.mouse_down_count != 0 {
                    self.mouse_down_count -= 1;
                }
            }
            glevent::Resized(w, h) => {
                self.window_size = (w as u32, h as u32);
                self.event_store.push_back(WindowResized(self.window_size));
            }
            glevent::Moved(x, y) => {
                self.window_pos = (x as i32, y as i32);
                self.event_store.push_back(WindowMoved(self.window_pos));
            }
            glevent::MouseWheel(x, y) => {
                self.event_store.push_back(MouseWheel(x as f32, y as f32));
            }
            glevent::ReceivedCharacter(c) => {
                last_char = Some(c);
            }
            glevent::KeyboardInput(glutin::ElementState::Pressed, code, virt)  => {
                let c = virt.and_then(keycode_to_char)
                            .or(last_char.take())
                            .or_else(|| self.code_to_char.get(&(code as usize))
                                                         .map(|a| *a));
                self.event_store.push_back(KeyPressed(code, c, virt));

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
                let c = virt.and_then(keycode_to_char)
                            .or_else(|| self.code_to_char.get(&(code as usize))
                                                         .map(|a| *a));
                self.event_store.push_back(KeyReleased(code, c, virt));
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
            glevent::Refresh => {  }
        }}
    }

    /// Produce a frame that has been cleared with a color.
    pub fn cleared_frame<C: Color>(&mut self, clear_color: C) -> Frame {
        Frame::new(&self.display,
                   self.color_program.clone(),
                   self.tex_program.clone(),
                   self.idx_cache.clone(),
                   self.tex_vtx_cache.clone(),
                   self.color_vtx_cache.clone(),
                   Some(clear_color.to_rgba()),
                   self.font_cache.clone())
    }

    /// Produce a frame that has not been cleared.
    pub fn frame(&mut self) -> Frame {
        Frame::new(&self.display,
                   self.color_program.clone(),
                   self.tex_program.clone(),
                   self.idx_cache.clone(),
                   self.tex_vtx_cache.clone(),
                   self.color_vtx_cache.clone(),
                   None,
                   self.font_cache.clone())
    }
}

#[allow(unused_variables)]
impl LuxCanvas for Frame {
    fn size(&self) -> (f32, f32) {
        use glium::Surface;
        let (w, h) = self.f.get_dimensions();
        (w as f32, h as f32)
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

    fn get_size_u(&self) -> (u32, u32) {
        self.window_size
    }

    fn get_size(&self) -> (Float, Float) {
        let (x, y) = self.get_size_u();
        (x as Float, y as Float)
    }

    fn is_focused(&self) -> bool {
        self.focused
    }

    fn mouse_pos_i(&self) -> (i32, i32) {
        self.mouse_pos
    }

    fn mouse_pos(&self) -> (f32, f32) {
        (self.mouse_pos.0 as f32, self.mouse_pos.1 as f32)
    }

    fn is_mouse_down(&self) -> bool {
        self.mouse_down_count != 0
    }

    fn events(&mut self) -> EventIterator {
        use std::mem::replace;
        self.process_events();
        EventIterator::from_deque(replace(&mut self.event_store, VecDeque::new()))
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
        &mut self.basis_matrix
    }

    fn current_matrix(&self) -> &[[f32; 4]; 4] {
        &self.basis_matrix
    }
}

impl Colored for Frame {
    fn get_color(&self) -> [f32; 4] {
        self.color
    }

    fn color<C: Color>(&mut self, color: C) -> &mut Frame {
        self.color = color.to_rgba();
        self
    }
}

impl HasDisplay for Window {
    fn borrow_display(&self) -> &glium::Display {
        &self.display
    }
}

impl HasFontCache for Window {
    fn font_cache(&self) -> RefMut<FontCache> {
        self.font_cache.borrow_mut()
    }
}

impl HasDisplay for Frame {
    fn borrow_display(&self) -> &glium::Display {
        &self.display
    }
}

impl HasFontCache for Frame {
    fn font_cache(&self) -> RefMut<FontCache> {
        self.font_cache.borrow_mut()
    }
}

impl HasSurface for Frame {
    type Out = glium::Frame;

    fn surface(&mut self) -> &mut Self::Out {
        &mut self.f
    }

    fn surface_and_texture_shader(&mut self) -> (&mut Self::Out, &glium::Program) {
        (&mut self.f, &*self.tex_program)
    }

    fn surface_and_color_shader(&mut self) -> (&mut Self::Out, &glium::Program) {
        (&mut self.f, &*self.color_program)
    }
}

impl HasDrawCache for Frame {
    fn color_draw_cache(&self) -> &Option<CachedColorDraw> {
        &self.color_draw_cache
    }
    fn tex_draw_cache(&self) -> &Option<CachedTexDraw> {
        &self.tex_draw_cache
    }

    fn color_draw_cache_mut(&mut self) -> &mut Option<CachedColorDraw> {
        &mut self.color_draw_cache
    }
    fn tex_draw_cache_mut(&mut self) -> &mut Option<CachedTexDraw> {
        &mut self.tex_draw_cache
    }
}

impl HasPrograms for Window {
    fn texture_shader(&self) -> &glium::Program {
        &*self.tex_program
    }

    fn color_shader(&self) -> &glium::Program {
        &*self.color_program
    }
}

impl HasPrograms for Frame {
    fn texture_shader(&self) -> &glium::Program {
        &*self.tex_program
    }

    fn color_shader(&self) -> &glium::Program {
        &*self.color_program
    }
}

impl Fetch<Vec<u16>> for Frame {
    fn fetch(&self) -> reuse_cache::Item<Vec<u16>> {
        let mut ret = self.idx_cache.get_or_else(|| vec![]);
        ret.clear();
        ret
    }
}

impl Fetch<Vec<TexVertex>> for Frame {
    fn fetch(&self) -> reuse_cache::Item<Vec<TexVertex>> {
        let mut ret = self.tex_vtx_cache.get_or_else(|| vec![]);
        ret.clear();
        ret
    }
}

impl Fetch<Vec<ColorVertex>> for Frame {
    fn fetch(&self) -> reuse_cache::Item<Vec<ColorVertex>> {
        let mut ret = self.color_vtx_cache.get_or_else(|| vec![]);
        ret.clear();
        ret
    }
}

impl DrawParamMod for Frame {
    fn draw_param_mod(&self) -> &DrawParamModifier {
        &self.draw_mod
    }
    fn draw_param_mod_mut(&mut self) -> &mut DrawParamModifier {
        &mut self.draw_mod
    }
}
