#![feature(phase, globs, unboxed_closures)]

#[phase(plugin)]
extern crate gfx_macros;
extern crate gfx;
extern crate render;
extern crate device;
extern crate glutin;
extern crate vecmath;
extern crate typemap;

pub use render::ProgramError;
pub use gfx::PrimitiveType;
pub use gfx::PrimitiveType::{
    Point,
    Line,
    LineStrip,
    TriangleList,
    TriangleStrip,
    TriangleFan
};

pub use gfx_integration::Vertex;
pub use canvas::{LuxCanvas, PrimitiveCanvas, Ellipse, Rectangle};
pub use interactive::*;
pub use interactive::Event::*;
pub use interactive::MouseButton::*;
pub use raw::{Colored, StackedColored, Transform, StackedTransform};
pub use glutin_window::Window;
pub use color::Color;
pub use texture::Texture;

mod interactive;
mod texture;
mod canvas;
mod raw;
mod gfx_integration;
mod glutin_window;
mod color;
pub mod colors;

#[deriving(Show)]
pub enum LuxError {
    WindowError(String),
    ShaderError(ProgramError)
}

pub trait LuxExtend {
    fn typemap(&self) -> &typemap::TypeMap;
    fn typemap_mut(&mut self) -> &mut typemap::TypeMap;
}

pub type LuxResult<A> = Result<A, LuxError>;
