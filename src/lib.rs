#![feature(plugin, unboxed_closures, unsafe_destructor, collections, core, hash, io)]
#![feature(std_misc, path)]

#![plugin(glium_macros)]
extern crate glium_macros;
extern crate glium;
extern crate glutin;
extern crate vecmath;
extern crate typemap;
extern crate image;
extern crate freetype;
extern crate "color" as ext_color;
extern crate texture_packer;

use std::error::{Error, FromError};
use std::old_io::IoError;

pub use gfx_integration::{ColorVertex, TexVertex};
pub use canvas::{LuxCanvas, PrimitiveCanvas, Ellipse, Rectangle, ContainedSprite};
pub use interactive::*;
pub use interactive::Event::*;
pub use interactive::MouseButton::*;
pub use raw::{Colored, StackedColored, Transform, StackedTransform};
pub use glutin_window::Window;
pub use color::{Color, rgb, rgba, hsv, hsva, hex_rgb, hex_rgba};
pub use sprite::{Sprite, SpriteLoader, NonUniformSpriteSheet, UniformSpriteSheet};
pub use figure::Figure;
pub use font::{FontCache, TextDraw, FontLoad, gen_sheet};

pub use glium::index_buffer::PrimitiveType;
pub use glium::index_buffer::PrimitiveType::*;
pub use image::ImageError;
pub use freetype::error::Error as FreetypeError;

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

pub type LuxResult<A> = Result<A, LuxError>;
#[derive(Debug)]
pub enum LuxError {
    WindowError(String),
    OpenGlError(String),
    ShaderError(glium::ProgramCreationError),
    FontError(FreetypeError, String),
    IoError(IoError),
    FontNotLoaded(String)
}

impl Error for LuxError {
    fn description(&self) -> &str {
        match self {
            &LuxError::WindowError(ref s) => &s[],
            &LuxError::OpenGlError(ref s) => &s[],
            &LuxError::ShaderError(ref e) => e.description(),
            &LuxError::FontError(_, ref s) => &s[],
            &LuxError::IoError(ref ioe) => ioe.description(),
            &LuxError::FontNotLoaded(ref s) => &s[],
        }
    }
}

impl FromError<FreetypeError> for LuxError {
    fn from_error(e: FreetypeError) -> LuxError {
        use std::fmt::Write;
        let mut bf = String::new();
        write!(&mut bf, "{}", e).unwrap();
        LuxError::FontError(e, bf)
    }
}

impl FromError<glium::ProgramCreationError> for LuxError {
    fn from_error(e: glium::ProgramCreationError) -> LuxError {
        LuxError::ShaderError(e)
    }
}

impl FromError<IoError> for LuxError {
    fn from_error(ioe: IoError) -> LuxError {
        LuxError::IoError(ioe)
    }
}

impl std::fmt::Display for LuxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            &LuxError::WindowError(ref s) => s.fmt(f),
            &LuxError::OpenGlError(ref s) => s.fmt(f),
            &LuxError::ShaderError(ref e) => e.fmt(f),
            &LuxError::FontError(ref e, _) => e.fmt(f),
            &LuxError::IoError(ref e) => e.fmt(f),
            &LuxError::FontNotLoaded(ref s) => s.fmt(f),
        }
    }
}

pub trait LuxExtend {
    fn typemap(&self) -> &typemap::TypeMap;
    fn typemap_mut(&mut self) -> &mut typemap::TypeMap;
}
