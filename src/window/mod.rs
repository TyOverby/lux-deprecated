use std::num::FloatMath;
use gfx::{
    ClearData,
    COLOR,
    Frame,
    ToSlice,
    DeviceHelper,
    Graphics,
    GlCommandBuffer,
    GlDevice
};
use super::{
    Color,
    Vertex,
    LovelyResult,
    Dummy
};
use super::color::{Color4, Color3};
use vecmath;

pub use self::gfx_integration as gfxi;
pub mod gfx_integration;

type Mat4f = [[f32, ..4], ..4];
type BaseColor = [f32, ..4];

pub struct Window {
    glutin_window: ::glutin::Window,
    graphics: Graphics<GlDevice, GlCommandBuffer>,
    program: ::device::Handle<u32, ::device::shade::ProgramInfo>,
    frame: Frame,

    basis_matrix: Mat4f,
    matrix_stack: Vec<Mat4f>,
    color_stack: Vec<BaseColor>,

    title: String,

    stored_rect: Option<Shape>
}


pub struct Shape {
    batch: gfx_integration::BasicBatch,
    color: Option<BaseColor>
}


impl Window {
    pub fn new() -> LovelyResult<Window> {
        ::glutin::Window::new()
            .map_err(|_| Dummy)
            .and_then(|w| {
                w.set_title("Lovely");
                unsafe { w.make_current(); }
                let mut device = GlDevice::new(|s| w.get_proc_address(s));
                let program = device.link_program(gfxi::VERTEX_SRC.clone(),
                                                  gfxi::FRAGMENT_SRC.clone());
                match program {
                    Ok(p) => Ok((w, device, p)),
                    Err(_) => Err(Dummy)
                }
            }).map(|(w, d, p)|{
                let graphics = Graphics::new(d);
                let (width, height) = w.get_inner_size().unwrap_or((0, 0));
                let mut basis = vecmath::mat4_id();
                basis[1][1] = -1.0;
                basis[3][0] = -1.0;
                basis[3][1] = 1.0;
                Window {
                    glutin_window: w,
                    graphics: graphics,
                    program: p,
                    frame: Frame::new(width as u16, height as u16),
                    matrix_stack: vec![],
                    color_stack: vec![[1.0,0.0,0.0,1.0]],
                    title: "Lovely".to_string(),
                    basis_matrix: basis,
                    stored_rect: None
                }
            })
    }

    pub fn clear<C: ToRgba>(&mut self, color: C) {
        self.graphics.clear(
            ClearData{
                color: color.to_rgba(),
                depth: 1.0,
                stencil: 0
            },
            COLOR,
            &Frame::new(50, 50))
    }

    pub fn render(&mut self) {
        self.graphics.end_frame();
        self.glutin_window.swap_buffers();
        self.matrix_stack.clear();

        let (w, h) = (self.w() as f32, self.h() as f32);
        let (sx, sy) = (2.0 / w, -2.0 / h);
        self.basis_matrix[0][0] = sx;
        self.basis_matrix[1][1] = sy;
        self.glutin_window.wait_events();
    }

    fn new_scope_transform(&mut self, mat: Mat4f) {
        let cur = self.current_matrix();
        self.matrix_stack.push(vecmath::col_mat4_mul(cur, mat));
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        let mut prod = vecmath::mat4_id();
        prod[0][0] = x;
        prod[1][1] = y;
        self.new_scope_transform(prod);
    }

    pub fn shear(&mut self, sx: f32, sy: f32) {
        let mut prod = vecmath::mat4_id();
        prod[1][0] = sx;
        prod[0][1] = sy;
        self.new_scope_transform(prod);
    }

    pub fn rotate(&mut self, theta: f32) {
        let mut prod = vecmath::mat4_id();
        let (c, s) = (theta.cos(), theta.sin());
        prod[0][0] = c;
        prod[0][1] = s;
        prod[1][0] = -s;
        prod[1][1] = c;
        self.new_scope_transform(prod);
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        let mut prod = vecmath::mat4_id();
        prod[3][0] = dx;
        prod[3][1] = dy;
        self.new_scope_transform(prod);
    }

    fn w(&self) -> i32 {
        match self.glutin_window.get_inner_size().unwrap() {
            (w, _) => w as i32
        }
    }

    fn h(&self) -> i32 {
        match self.glutin_window.get_inner_size().unwrap() {
            (_, h) => h as i32
        }
    }

    fn current_matrix(&self) -> [[f32, ..4], ..4] {
        if self.matrix_stack.len() == 0 {
            return self.basis_matrix;
        } else {
            return self.matrix_stack[self.matrix_stack.len()-1];
        }
    }

    pub fn push_matrix(&mut self) {
        let mat = self.current_matrix();
        self.matrix_stack.push(mat);
    }

    pub fn pop_matrix(&mut self) {
        self.matrix_stack.pop();
    }

    pub fn stamp_shape(&mut self, vertices: &[Vertex]) -> Shape {
        let mesh = self.graphics.device.create_mesh(vertices);
        let slice = mesh.to_slice(::gfx::TriangleFan);
        let state = ::gfx::DrawState::new();
        let batch: gfx_integration::BasicBatch =
            self.graphics.make_batch(&self.program, &mesh, slice, &state).unwrap();
        Shape {
            batch: batch,
            color: None
        }
    }

    pub fn draw_shape(&mut self, shape: &Shape) {
        let mat = self.current_matrix();
        let params = gfx_integration::Params {
            transform: mat,
            color: shape.color.unwrap_or([1.0, 1.0, 0.0, 1.0])
        };

        self.graphics.draw(&shape.batch, &params, &self.frame)
    }
}

#[allow(unused_variables)]
impl super::LovelyCanvas<()> for Window {
    fn width(&self) -> i32 {
        match self.glutin_window.get_inner_size().unwrap() {
            (w, _) => w as i32
        }
    }

    fn height(&self) -> i32 {
        match self.glutin_window.get_inner_size().unwrap() {
            (_, h) => h as i32
        }
    }
    fn draw_rect(&mut self, pos: super::Vec2f, size: super::Vec2f) {
        if self.stored_rect.is_none() {
            let vertex_data = [
                Vertex{ pos: [0.0, 0.0], tex: [0.0, 0.0] },
                Vertex{ pos: [1.0, 0.0], tex: [1.0, 0.0] },
                Vertex{ pos: [1.0, 1.0], tex: [1.0, 1.0] },
                Vertex{ pos: [0.0, 1.0], tex: [0.0, 1.0] },
            ];
            let shape = self.stamp_shape(vertex_data);
            self.stored_rect = Some(shape);
        }
        let shape = self.stored_rect.unwrap();
        let (x, y) = pos;
        let (w, h) = size;
        self.with_translate(x, y, |window| {
            window.with_scale(w, h, |window| {
                window.draw_shape(&shape)
            });
        });
    }
    fn draw_border_rect(&mut self, pos: super::Vec2f, size: super::Vec2f, border_size: f32) {
        unimplemented!();
    }

    fn draw_circle(&mut self, pos: super::Vec2f, radius: f32) {
        unimplemented!();
    }
    fn draw_border_circle(&mut self, pos: super::Vec2f, radius: f32, border_size: f32) {
        unimplemented!();
    }

    fn draw_elipse(&mut self, pos: super::Vec2f, size: super::Vec2f) {
        unimplemented!();
    }
    fn draw_border_elipse(&mut self, pos: super::Vec2f, size: super::Vec2f, border_size: f32) {
        unimplemented!();
    }

    fn draw_line(&mut self, positions: &Vec<super::Vec2f>, line_size: f32) {
        unimplemented!();
    }
    fn draw_arc(&mut self, pos: super::Vec2f, radius: f32, angle1: f32, angle2: f32) {
        unimplemented!();
    }

    fn draw_point(&mut self, pos: super::Vec2f) {
        unimplemented!();
    }

    fn with_color(&mut self, color: Color, f: |&mut Window| -> ()) {
        unimplemented!();
    }
    fn with_border_color(&mut self, color: Color, f: |&mut Window| -> ()) {
        unimplemented!();
    }
    fn with_rotation(&mut self, theta: f32, f: |&mut Window| -> ()) {
        self.rotate(theta);
        f(self);
        self.pop_matrix();
    }
    fn with_translate(&mut self, dx: f32, dy: f32, f: |&mut Window| -> ()) {
        self.translate(dx, dy);
        f(self);
        self.pop_matrix();
    }
    fn with_scale(&mut self, sx: f32, sy: f32, f: |&mut Window| -> ()) {
        self.scale(sx, sy);
        f(self);
        self.pop_matrix();
    }
    fn with_shear(&mut self, sx: f32, sy: f32, f: |&mut Window| -> ()) {
        self.shear(sx, sy);
        f(self);
        self.pop_matrix();
    }

    fn draw<T: super::Drawable<()>>(&mut self, figure: T) {
        unimplemented!();
    }

    fn draw_text(&mut self, pos: super::Vec2f, text: &str) {
        unimplemented!();
    }
}

impl super::LovelyWindow for Window {
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
            .map(|(a,b)| (a as u32, b as u32))
            .unwrap_or((0,0))
    }
}

pub trait ToRgba {
    fn to_rgba(self) -> [f32, ..4];
}

impl ToRgba for super::color::Rgb<f32> {
    fn to_rgba(self) -> [f32, ..4] {
        match self.into_fixed() {
            [r,g,b] => [r,g,b,1.0]
        }
    }
}

impl ToRgba for super::color::Rgb<u8> {
    fn to_rgba(self) -> [f32, ..4] {
        match self.into_fixed() {
            [r,g,b] => [r as f32 / 255u as f32,
                        g as f32 / 255u as f32,
                        b as f32 / 255u as f32,
                        1.0]
        }
    }
}

impl ToRgba for super::color::Rgba<f32> {
    fn to_rgba(self) -> [f32, ..4] {
        self.into_fixed()
    }
}
