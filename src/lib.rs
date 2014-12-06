#![feature(phase, globs, unboxed_closures, if_let, tuple_indexing)]

#[phase(plugin)]
extern crate gfx_macros;
extern crate gfx;
extern crate render;
extern crate device;
extern crate glutin;
extern crate vecmath;

pub use gfx_integration::Vertex;

pub use render::ProgramError;
pub use gfx::PrimitiveType;
pub use gfx::PrimitiveType::{ Point, Line, LineStrip,
               TriangleList, TriangleStrip, TriangleFan };

pub use canvas::{LuxCanvas, BasicShape};
pub use window::*;
pub use window::LuxEvent::*;
pub use window::LuxEvent::*;
pub use window::MouseButton::*;
pub use raw::{Transform, StackedTransform};
pub use glutin_window::Window;
pub use color::Color;

mod window;
mod canvas;
mod raw;
mod gfx_integration;
mod glutin_window;
mod color;

#[deriving(Show)]
pub enum LuxError {
    WindowError(String),
    ShaderError(ProgramError)
}

pub trait Drawable {
    fn draw<C: LuxCanvas>(&self, &mut C);
}

pub type LuxResult<A> = Result<A, LuxError>;
