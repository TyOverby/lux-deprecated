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

mod gfx_integration;

pub struct Window {
    glutin_window: ::glutin::Window,
    graphics: Graphics<GlDevice, GlCommandBuffer>,
    program: ::device::Handle<u32, ::device::shade::ProgramInfo>,
    frame: Frame,
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
                            frame: Frame::new(width as u16, height as u16)
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
            Vertex{pos: [-0.5, -0.5], color: [1.0, 0.0, 0.0], tex: [0.0, 0.0]},
            Vertex{pos: [ 0.5, -0.5], color: [0.0, 1.0, 0.0], tex: [0.0, 0.0]},
            Vertex{pos: [ 0.0,  0.5], color: [0.0, 0.0, 1.0], tex: [0.0, 0.0]}
        ];
        let mesh = self.graphics.device.create_mesh(vertex_data);
        let slice = mesh.to_slice(TriangleList);
        let state = ::gfx::DrawState::new();
        let batch: RefBatch<(), ()> =
            self.graphics.make_batch(&self.program, &mesh, slice, &state).unwrap();
        self.graphics.draw(&batch, &(), &self.frame)
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
