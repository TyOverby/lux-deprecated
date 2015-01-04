#![feature(phase, globs, unboxed_closures, unsafe_destructor)]

#[phase(plugin)]
extern crate glium_macros;
extern crate glium;
extern crate glutin;
extern crate vecmath;
extern crate typemap;

pub use gfx_integration::Vertex;
pub use canvas::{LuxCanvas, PrimitiveCanvas, Ellipse, Rectangle};
pub use interactive::*;
pub use interactive::Event::*;
pub use interactive::MouseButton::*;
pub use raw::{Colored, StackedColored, Transform, StackedTransform};
pub use glutin_window::Window;
pub use color::Color;

pub use glium::index_buffer::PrimitiveType;
pub use glium::index_buffer::PrimitiveType::*;

mod interactive;
mod texture;
mod canvas;
mod raw;
mod gfx_integration;
mod glutin_window;
mod color;
pub mod colors;

#[derive(Show)]
pub enum LuxError {
    WindowError(String),
    OpenGlError(String),
    ShaderError(glium::ProgramCreationError)
}

pub trait LuxExtend {
    fn typemap(&self) -> &typemap::TypeMap;
    fn typemap_mut(&mut self) -> &mut typemap::TypeMap;
}

pub type LuxResult<A> = Result<A, LuxError>;
