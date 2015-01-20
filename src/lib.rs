#![feature(plugin, unboxed_closures, unsafe_destructor)]
#![allow(unstable)]

#[plugin]
extern crate glium_macros;
extern crate glium;
extern crate glutin;
extern crate vecmath;
extern crate typemap;
extern crate image;

pub use gfx_integration::{ColorVertex, TexVertex};
pub use canvas::{LuxCanvas, PrimitiveCanvas, Ellipse, Rectangle};
pub use interactive::*;
pub use interactive::Event::*;
pub use interactive::MouseButton::*;
pub use raw::{Colored, StackedColored, Transform, StackedTransform};
pub use glutin_window::Window;
pub use color::Color;
pub use sprite::{Sprite, SpriteLoader};

pub use glium::index_buffer::PrimitiveType;
pub use glium::index_buffer::PrimitiveType::*;
pub use image::ImageError;

mod interactive;
mod canvas;
mod raw;
mod gfx_integration;
mod glutin_window;
mod color;
mod sprite;
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
