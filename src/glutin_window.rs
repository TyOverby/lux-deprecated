use std::num::FloatMath;
use std::vec::MoveItems;
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
use render::state::BlendAlpha;
use super::{
    Color,
    Vertex,
    LuxResult,
    WindowError,
    ShaderError
};
use super::{LuxCanvas, LuxWindow, LuxRaw, LuxEvent};
use vecmath;

use super::gfx_integration;
use super::gfx_integration as gfxi;

type Mat4f = [[f32, ..4], ..4];
type BaseColor = [f32, ..4];

pub struct Window {
    // CANVAS
    glutin_window: ::glutin::Window,
    graphics: Graphics<GlDevice, GlCommandBuffer>,
    program: ::device::Handle<u32, ::device::shade::ProgramInfo>,
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

    // EVENT
    event_store: Vec<LuxEvent>,
    mouse_pos: (i32, i32),
    window_pos: (i32, i32),
    window_size: (u32, u32),
    focused: bool,
    mouse_down_count: u8,
    events_since_last_render: bool
}


pub struct Shape {
    batch: gfx_integration::BasicBatch,
    color: Option<BaseColor>
}

impl Window {
    pub fn new() -> LuxResult<Window> {
        let window = try!(::glutin::Window::new().map_err(|e| {
            match e {
                ::glutin::OsError(s) => WindowError(s)
            }
        }));
        window.set_title("Lux");
        unsafe { window.make_current(); }
        let mut device = GlDevice::new(|s| window.get_proc_address(s));
        let (vtx, frag) = (gfxi::VERTEX_SRC.clone(), gfxi::FRAGMENT_SRC.clone());
        let program = try!(device.link_program(vtx, frag)
                           .map_err(super::ShaderError));
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
            draw_state: DrawState::new().blend(BlendAlpha),
            frame: Frame::new(width as u16, height as u16),
            matrix_stack: vec![],
            color_stack: vec![[1.0,0.0,0.0,1.0]],
            title: "Lux".to_string(),
            basis_matrix: basis,
            stored_rect: None,
            stored_circle: None,
            event_store: vec![],
            mouse_pos: (0, 0),
            window_pos: (0, 0),
            window_size: (width as u32, height as u32),
            focused: true,
            mouse_down_count: 0,
            events_since_last_render: false
        };
        Ok(window)
    }

    pub fn process_events(&mut self) {
        use glutin;
        self.events_since_last_render = true;
        fn t_mouse(button: glutin::MouseButton) -> super::MouseButton {
            match button {
                glutin::LeftMouseButton => super::Left,
                glutin::RightMouseButton => super::Right,
                glutin::MiddleMouseButton => super::Middle,
                glutin::OtherMouseButton(a) => super::OtherMouseButton(a)
            }
        }
        let mut last_char = None;
        for event in self.glutin_window.poll_events() { match event {
            glutin::MouseMoved((x, y)) => {
                self.mouse_pos = (x as i32, y as i32);
                self.event_store.push(super::MouseMoved((x as i32, y as i32)))
            }
            glutin::MouseInput(glutin::Pressed, button) => {
                self.event_store.push(super::MouseDown(t_mouse(button)));
                self.mouse_down_count += 1;
            }
            glutin::MouseInput(glutin::Released, button) => {
                self.event_store.push(super::MouseUp(t_mouse(button)));
                self.mouse_down_count -= 1;
            }
            glutin::Resized(w, h) => {
                self.window_size = (w as u32, h as u32);
                self.event_store.push(super::WindowResized(self.window_size));
            }
            glutin::Moved(x, y) => {
                self.window_pos = (x as i32, y as i32);
                self.event_store.push(super::WindowMoved(self.window_pos));
            }
            glutin::MouseWheel(i) => {
                self.event_store.push(super::MouseWheel(i as i32));
            }
            glutin::ReceivedCharacter(c) => {
                last_char = Some(c);
            }
            glutin::KeyboardInput(glutin::Pressed, code, virt)  => {
                let c = virt.and_then(super::keycode_to_char)
                            .or_else(|| last_char.take());
                self.event_store.push( super::KeyPressed(code, c, virt));
            }
            glutin::KeyboardInput(glutin::Released, code, virt) => {
                let c = virt.and_then(super::keycode_to_char)
                            .or_else(|| last_char.take());
                self.event_store.push( super::KeyReleased(code, c, virt));
            }
            glutin::Focused(f) => {
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

        let(wi, hi) = self.size();
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

    fn draw_rect(&mut self, pos: (f32, f32), size: (f32, f32)) {
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
            let shape = self.stamp_shape(vertex_data, super::TriangleList);
            self.stored_rect = Some(shape);
        }
        let (x, y) = pos;
        let (w, h) = size;
        self.push_matrix();
        self.translate(x, y);
        self.scale(w, h);
        // This is safe because draw_shape does *not* mutate the shape itself.
        let shape = unsafe { transmute(self.stored_rect.as_ref().unwrap()) };
        self.draw_shape(shape);
        self.pop_matrix();
    }

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
    }

    fn draw_circle(&mut self, pos: (f32, f32), radius: f32) {
        self.draw_elipse(pos, (radius, radius));
    }

    fn draw_border_circle(&mut self, pos: (f32, f32), radius: f32, border_size: f32) {
        unimplemented!();
    }

    fn draw_elipse(&mut self, pos: (f32, f32), size: (f32, f32)) {
        use std::num::FloatMath;
        use std::intrinsics::transmute;
        if self.stored_circle.is_none() {
            let mut vertex_data = vec![];
            let pi = Float::pi();
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
        self.push_matrix();
        self.translate(x+sx, y+sy);
        self.scale(sx, sy);
        self.draw_shape(shape);
        self.pop_matrix();
    }
    fn draw_border_elipse(&mut self, pos: (f32, f32), size: (f32, f32), border_size: f32) {
        unimplemented!();
    }

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
        self.draw_rect((0.0,0.0), (1.0,1.0));
        self.pop_matrix();
    }

    fn draw_lines(&mut self, positions: &[(f32, f32)], line_size: f32) {
        if positions.len() <= 1 { return; }
        let l_mod = line_size / 2.0;
        for i in range(0, positions.len() - 1) {
            let (x1, y1) = positions[i];
            let (x2, y2) = positions[i+1];
            self.draw_line((x1, y1), (x2, y2), line_size);
            self.draw_circle((x1 - l_mod, y1 - l_mod), l_mod);
        }
        let (lx, ly) = positions[positions.len()-1];
        self.draw_circle((lx - l_mod, ly - l_mod), l_mod);
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
}

impl LuxRaw for Window {
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

    fn push_matrix(&mut self) {
        let c = *self.current_matrix();
        self.matrix_stack.push(c);
    }

    fn pop_matrix(&mut self) {
        self.matrix_stack.pop();
    }
}
