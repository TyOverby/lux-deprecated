#![feature(phase, globs, unboxed_closures, if_let)]

#[phase(plugin)]
extern crate gfx_macros;
extern crate gfx;
extern crate render;
extern crate device;
extern crate glutin;
extern crate vecmath;

pub use gfx_integration::Vertex;

pub use render::{ ProgramError, ErrorVertex, ErrorFragment, ErrorLink };
pub use gfx::{ PrimitiveType, Point, Line, LineStrip,
               TriangleList, TriangleStrip, TriangleFan };

pub use canvas::LuxCanvas;
pub use window::*;
pub use raw::LuxRaw;
pub use glutin_window::Window;
pub use color::Color;

mod window;
mod canvas;
mod raw;
mod gfx_integration;
mod glutin_window;
mod color;

pub type LuxResult<A> = Result<A, LuxError>;

#[deriving(Show)]
pub enum LuxError {
    WindowError(String),
    ShaderError(ProgramError)
}

pub trait Drawable {
    fn draw<C: LuxCanvas>(&self, &mut C);
}
