#![feature(phase)]

#[phase(plugin)]
extern crate gfx_macros;
extern crate gfx;
extern crate render;
extern crate device;

extern crate glutin;

extern crate vecmath;

extern crate "color" as color_lib;

pub use color_lib as color;
pub use window::gfx_integration::Vertex;

pub mod window;

pub type Vec2f = (f32, f32);
pub type Color = color::Rgba<f32>;

pub enum DrawPrimitive {
    Points,
    Lines,
    LinesStrip,
    Triangles,
    TrianglesStrip,
    TrianglesFan,
    Quads
}

pub trait Drawable<Tex> {
    fn primitive(&self) -> DrawPrimitive;
    fn vertices(&self) -> &Vec<Vertex>;
    fn texture(&self) -> Option<&Tex>;
    fn color(&self) -> Option<Color>;
}

pub trait Lovely<Tex> {
    fn width(&self) -> i32;
    fn height(&self) -> i32;

    fn draw_rect(&mut self, pos: Vec2f, size: Vec2f);
    fn draw_border_rect(&mut self, pos: Vec2f, size: Vec2f, border_size: f32);

    fn draw_circle(&mut self, pos: Vec2f, radius: f32);
    fn draw_border_circle(&mut self, pos: Vec2f, radius: f32, border_size: f32);

    fn draw_elipse(&mut self, pos: Vec2f, size: Vec2f);
    fn draw_border_elipse(&mut self, pos: Vec2f, size: Vec2f, border_size: f32);

    fn draw_line(&mut self, positions: &Vec<Vec2f>, line_size: f32);
    fn draw_arc(&mut self, pos: Vec2f, radius: f32, angle1: f32, angle2: f32);

    fn draw_point(&mut self, pos: Vec2f);

    fn with_color(&mut self, color: Color, f: |&mut Self| -> ());
    fn with_border_color(&mut self, color: Color, f: |&mut Self| -> ());
    fn with_rotation(&mut self, rotation: f32, f: |&mut Self| -> ());
    fn with_translation(&mut self, dx: f32, dy: f32, f: |&mut Self| -> ());
    fn with_scale(&mut self, scale_x: f32, scale_y: f32, f: |&mut Self| -> ());
    fn with_shear(&mut self, sx: f32, sy: f32, f: |&mut Self| -> ());

    fn draw<T: Drawable<Tex>>(&mut self, figure: T);

    fn draw_text(&mut self, pos: Vec2f, text: &str);
}
