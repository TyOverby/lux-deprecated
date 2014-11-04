use gfx::{
    ClearData,
    COLOR,
    Frame,
    TriangleList,
    ToSlice,
    DeviceHelper,
    Graphics,
    GlCommandBuffer,
    GlDevice
};
use gfx::batch::RefBatch;
use super::Color;
use super::color::Color4;
use super::color::Color3;
use super::Vertex;
use vecmath;

pub mod gfx_integration;

type Mat4f = [[f32, ..4], ..4];

pub struct Window {
    glutin_window: ::glutin::Window,
    graphics: Graphics<GlDevice, GlCommandBuffer>,
    program: ::device::Handle<u32, ::device::shade::ProgramInfo>,
    frame: Frame,

    basis_matrix: Mat4f,
    matrix_stack: Vec<Mat4f>,

    rect_batch: Option<gfx_integration::BasicBatch>
}


pub struct Shape {
    batch: gfx_integration::BasicBatch,
    color: Option<super::color::Rgba<f32>>
}

impl Window {
    pub fn new() -> Option<Window> {
        let glutin_window = ::glutin::Window::new();
        match glutin_window {
            Ok(w) => {
                w.set_title("Lovely");
                unsafe { w.make_current(); }
                let mut device = ::gfx::GlDevice::new(|s| w.get_proc_address(s));

                match device.link_program(gfx_integration::VERTEX_SRC.clone(),
                                          gfx_integration::FRAGMENT_SRC.clone()) {
                    Ok(program) => {
                        let graphics = Graphics::new(device);
                        let (width, height) = w.get_inner_size().unwrap();
                        Some(Window{
                            glutin_window: w,
                            graphics: graphics,
                            program: program,
                            frame: Frame::new(width as u16, height as u16),
                            basis_matrix:
                                 [[1.0, 0.0, 0.0, 0.0],
                                 [0.0,-1.0, 0.0, 0.0],
                                 [0.0, 0.0, 1.0, 0.0],
                                 [-1.0, 1.0, 0.0, 1.0]],
                            matrix_stack: vec![],
                            rect_batch: None
                        })
                    }
                    Err(_) => None
                }
            }
            Err(_) => None
        }
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
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        let mat = self.current_matrix();
        let mut prod = vecmath::mat4_id();
        prod[0][0] = x;
        prod[1][1] = y;
        self.matrix_stack.push(vecmath::col_mat4_mul(mat, prod));
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

    fn current_matrix_mut(&mut self) -> &mut [[f32, ..4], ..4] {
        let len = self.matrix_stack.len();
        if len == 0 {
            return &mut self.basis_matrix;
        } else {
            return self.matrix_stack.get_mut(len - 1);
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
        let (w, h) = (self.w() as f32, self.h() as f32);
        let (sx, sy) = (2.0 / w, -2.0 / h);
        let data = gfx_integration::Params {
            transform: [[sx , 0.0, 0.0, 0.0],
                        [0.0,  sy, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [-1.0,1.0, 0.0, 1.0]],
            color: [1.0, 0.0, 0.0, 1.0]
        };
        Shape {
            batch: batch,
            color: None
        }
    }

    pub fn draw_shape(&mut self, shape: &Shape) {
        let mut mat = self.current_matrix();

        let params = gfx_integration::Params {
            transform: mat,
            color: shape.color.map_or([1.0, 1.0, 0.0, 1.0], |c| c.to_rgba())
        };

        self.graphics.draw(&shape.batch, &params, &self.frame)
    }
}

#[allow(unused_variables)]
impl super::Lovely<()> for Window {
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
        let vertex_data = [
            Vertex{ pos: [0.0, 0.0], tex: [0.0, 0.0] },
            Vertex{ pos: [5.0, 0.0], tex: [1.0, 0.0] },
            Vertex{ pos: [5.0, 5.0], tex: [1.0, 1.0] },
            Vertex{ pos: [0.0, 5.0], tex: [0.0, 1.0] },
        ];
        let shape = self.stamp_shape(vertex_data);
        self.draw_shape(&shape)
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
    fn with_rotation(&mut self, rotation: f32, f: |&mut Window| -> ()) {
        unimplemented!();
    }
    fn with_translation(&mut self, translation: f32, f: |&mut Window| -> ()) {
        unimplemented!();
    }
    fn with_scale(&mut self, scale: f32, f: |&mut Window| -> ()) {
        unimplemented!();
    }
    fn with_shear(&mut self, shear: super::Vec2f, f: |&mut Window| -> ()) {
        unimplemented!();

    }

    fn draw<T: super::Drawable<()>>(&mut self, figure: T) {
        unimplemented!();
    }

    fn draw_text(&mut self, pos: super::Vec2f, text: &str) {
        unimplemented!();
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
