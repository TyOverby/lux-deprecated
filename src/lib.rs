#![feature(plugin, unboxed_closures, unsafe_destructor)]
#![allow(unstable)]

#[plugin]
extern crate glium_macros;
extern crate glium;
extern crate glutin;
extern crate vecmath;
extern crate typemap;
extern crate image;
extern crate freetype;
extern crate "color" as ext_color;
extern crate texture_packer;

pub use gfx_integration::{ColorVertex, TexVertex};
pub use canvas::{LuxCanvas, PrimitiveCanvas, Ellipse, Rectangle};
pub use interactive::*;
pub use interactive::Event::*;
pub use interactive::MouseButton::*;
pub use raw::{Colored, StackedColored, Transform, StackedTransform};
pub use glutin_window::Window;
pub use color::{Color, rgb, rgba, hsv, hsva};
pub use sprite::{Sprite, SpriteLoader, NonUniformSpriteSheet, UniformSpriteSheet};
pub use figure::Figure;
pub use font::{char_to_img, merge_all};

pub use glium::index_buffer::PrimitiveType;
pub use glium::index_buffer::PrimitiveType::*;
pub use image::ImageError;

mod interactive;
mod figure;
mod canvas;
mod raw;
mod gfx_integration;
mod glutin_window;
mod color;
mod sprite;
mod font;
pub mod colors;

#[derive(Debug)]
pub enum LuxError {
    WindowError(String),
    OpenGlError(String),
    ShaderError(glium::ProgramCreationError)
}

impl std::fmt::Display for LuxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            &LuxError::WindowError(ref s) => s.fmt(f),
            &LuxError::OpenGlError(ref s) => s.fmt(f),
            &LuxError::ShaderError(ref e) => e.fmt(f)
        }
    }
}

pub trait LuxExtend {
    fn typemap(&self) -> &typemap::TypeMap;
    fn typemap_mut(&mut self) -> &mut typemap::TypeMap;
}

pub type LuxResult<A> = Result<A, LuxError>;
