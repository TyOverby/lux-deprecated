#![feature(phase)]
#![feature(unboxed_closures)]

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
pub use window::LuxWindow;
pub use raw::LuxRaw;
pub use glutin_window::Window;

mod window;
mod canvas;
mod raw;
mod gfx_integration;
mod glutin_window;

pub trait Color {
    fn to_rgba(self) -> [f32, ..4];
}

#[deriving(Show)]
pub enum LuxError {
    WindowError(String),
    ShaderError(ProgramError)
}

pub type LuxResult<A> = Result<A, LuxError>;

pub trait Drawable {
    fn draw<C: LuxCanvas>(&self, &mut C);
}


impl Color for [f32, ..4] {
    fn to_rgba(self) -> [f32, ..4] {
        self
    }
}

impl Color for [f32, ..3] {
    fn to_rgba(self) -> [f32, ..4] {
        match self {
            [r,g,b] => [r,g,b,1.0]
        }
    }
}
