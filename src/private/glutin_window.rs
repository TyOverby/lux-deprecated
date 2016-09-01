use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use glutin;
use vecmath;
use glium;
use poison_pool;

use super::interactive::keycodes::VirtualKeyCode;
use super::accessors::{
    HasDrawCache,
    HasPrograms,
    HasDisplay,
    HasSurface,
    DrawParamMod,
    Fetch,
};

use super::interactive::{EventIterator, AbstractKey, Event, Interactive};
use super::gfx_integration::{ColorVertex, TexVertex};
use super::canvas::Canvas;
use super::color::Color;
use super::raw::{Colored, Transform};
use super::error::LuxResult;
use super::shaders::{gen_texture_shader, gen_color_shader};
use super::primitive_canvas::{
    PrimitiveCanvas,
    CachedColorDraw,
    CachedTexDraw,
    DrawParamModifier
};
use super::types::{Float, Idx};

type Mat4f = [[f32; 4]; 4];
type BaseColor = [f32; 4];

/// A set of options that can be applied to a window
#[derive(Clone, PartialEq, Eq)]
pub struct WindowOptions {
    /// The size of the window in pixels.
    pub dimensions: (u32, u32),
    /// The title displayed on the top of the window.
    pub title: String,
    // pub fullscreen: bool,
    /// If Vsync is enabled
    pub vsync: bool,
    /// The number of multisampling passes. Must be a power of 2.
    pub multisampling: u16,
    /// True if the window should be transparent.
    pub transparent: bool,
    /// True if the window should have no border or title-bar.
    pub decorations: bool
}

/// A 1 to 1 correlation with a window shown on your desktop.
///
/// Lux uses Glutin for the window implementation.
pub struct Window {
    // OPTIONS
    options: WindowOptions,
    // CANVAS
    display: glium::Display,
    color_program: Rc<glium::Program>,
    tex_program: Rc<glium::Program>,
    closed: bool,

    // WINDOW
    title: String,

    // CACHES
    idx_cache: poison_pool::PoisonPool<Vec<Idx>>,
    tex_vtx_cache: poison_pool::PoisonPool<Vec<TexVertex>>,
    color_vtx_cache: poison_pool::PoisonPool<Vec<ColorVertex>>,

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
    idx_cache: poison_pool::PoisonPool<Vec<Idx>>,
    tex_vtx_cache: poison_pool::PoisonPool<Vec<TexVertex>>,
    color_vtx_cache: poison_pool::PoisonPool<Vec<ColorVertex>>,

    // Raw
    basis_matrix: Mat4f,
    color: [f32; 4],

    // Misc
    draw_mod: DrawParamModifier,
}


impl Frame {
    fn new(display: &glium::Display,
           color_program: Rc<glium::Program>,
           tex_program: Rc<glium::Program>,
           idx_cache: poison_pool::PoisonPool<Vec<Idx>>,
           tex_vtx_cache: poison_pool::PoisonPool<Vec<TexVertex>>,
           color_vtx_cache: poison_pool::PoisonPool<Vec<ColorVertex>>,
           clear_color: Option<[f32; 4]>) -> Frame {
        use glium::Surface;

        let mut frm = display.draw();
        if let Some(c) = clear_color {
            frm.clear_color(c[0],c[1],c[2],c[3]);
        }
        frm.clear_stencil(0);

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
            draw_mod: DrawParamModifier::new()
        }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        self.flush_draw().unwrap();
        self.f.set_finish().unwrap();
    }
}

impl WindowOptions {
    fn into_window_builder(self) -> glium::glutin::WindowBuilder<'static> {
        let WindowOptions {
            dimensions,
            title,
            //fullscreen,
            vsync,
            multisampling,
            transparent,
            decorations
        } = self;

        let builder = glium::glutin::WindowBuilder::new()
            .with_dimensions(dimensions.0, dimensions.1)
            .with_title(title)
            .with_multisampling(multisampling)
            .with_transparency(transparent)
            .with_decorations(decorations);
        if vsync {
            builder.with_vsync()
        } else { builder }
    }
}

impl Default for WindowOptions {
    fn default() -> WindowOptions {
        WindowOptions {
            dimensions: (800, 500),
            title: "Lux".to_string(),
            multisampling: 1,
            vsync: true,
            transparent: false,
            decorations: true
        }
    }
}

impl Window {
    /// Creates a new lux Window with the provided window settings.
    pub fn new(options: WindowOptions) -> LuxResult<Window> {
        use glium::DisplayBuild;
        let window_builder = options.clone().into_window_builder();

        let display = try!(window_builder.build_glium());

        let color_program = try!(gen_color_shader(&display));
        let tex_program = try!(gen_texture_shader(&display));

        let (width, height): (u32, u32) = display.get_framebuffer_dimensions();

        let window = Window {
            options: options,
            display: display,
            color_program: Rc::new(color_program),
            tex_program: Rc::new(tex_program),
            closed: false,
            title: "Lux".to_string(),
            idx_cache: poison_pool::PoisonPool::new(4, || vec![]),
            tex_vtx_cache: poison_pool::PoisonPool::new(4, || vec![]),
            color_vtx_cache: poison_pool::PoisonPool::new(4, || vec![]),
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
        };

        Ok(window)

    }
    /// Constructs a new window with the default settings.
    pub fn new_with_defaults() -> LuxResult<Window> {
        Window::new(Default::default())
    }

    /// Executes a closure that can modify the window settings.
    ///
    /// Changes to the window are applied after the closure is done executing.
    pub fn change_options<F: FnOnce(&mut WindowOptions)>(&mut self, f: F) -> LuxResult<()> {
        use glium::DisplayBuild;
        let copy = self.options.clone();
        f(&mut self.options);
        if copy != self.options {
            try!(self.options.clone().into_window_builder().rebuild_glium(&self.display));
        }
        Ok(())
    }

    /// Add the events from an iterator of events back to the internal event queue.
    pub fn restock_events<I: DoubleEndedIterator<Item=Event>>(&mut self, mut i: I) {
        while let Some(e) = i.next_back() {
            self.event_store.push_front(e);
        }
    }

    /// Query the underlying window system for events and add them to the
    /// the interal event queue.
    ///
    /// This function is automatically called by `is_open()`, `events()`.
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
            glevent::MouseMoved(x, y) => {
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
            glevent::MouseWheel(wheel_delta, _) => {
                // TODO: remove this when this code breaks.
                let wheel_delta = match wheel_delta {
                    glutin::MouseScrollDelta::LineDelta(x, y) => MouseScrollDelta::LineDelta(x, y),
                    glutin::MouseScrollDelta::PixelDelta(x, y) => MouseScrollDelta::PixelDelta(x, y)
                };
                self.event_store.push_back(MouseWheel(wheel_delta));
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
            glevent::DroppedFile(buf) => {
                self.event_store.push_back(FileDropped(buf))
            }
            glevent::Awakened => {
                // TODO: handle this
            }
            glevent::Refresh => {
                // TODO: handle this
            }
            glevent::Suspended(_) => {

            }
            glevent::Touch(_) => {
                // TODO: -- high priority -- handle this.
            }
            glevent::TouchpadPressure(_, _) => {

            }
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
                   Some(clear_color.to_rgba()))
    }

    /// Produce a frame that has not been cleared.
    pub fn frame(&mut self) -> Frame {
        Frame::new(&self.display,
                   self.color_program.clone(),
                   self.tex_program.clone(),
                   self.idx_cache.clone(),
                   self.tex_vtx_cache.clone(),
                   self.color_vtx_cache.clone(),
                   None)
    }
}

#[allow(unused_variables)]
impl Canvas for Frame {
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

impl HasDisplay for Frame {
    fn borrow_display(&self) -> &glium::Display {
        &self.display
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

impl Fetch<Vec<Idx>> for Frame {
    fn fetch(&self) -> poison_pool::Item<Vec<Idx>> {
        let mut ret = self.idx_cache.get_or_else(|| vec![]);
        ret.clear();
        ret
    }
}

impl Fetch<Vec<TexVertex>> for Frame {
    fn fetch(&self) -> poison_pool::Item<Vec<TexVertex>> {
        let mut ret = self.tex_vtx_cache.get_or_else(|| vec![]);
        ret.clear();
        ret
    }
}

impl Fetch<Vec<ColorVertex>> for Frame {
    fn fetch(&self) -> poison_pool::Item<Vec<ColorVertex>> {
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
