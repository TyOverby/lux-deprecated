#![feature(plugin, unboxed_closures, unsafe_destructor, collections)]
#![feature(slice_patterns, debug_builders)]

#[macro_use] extern crate glium;
extern crate glutin;
extern crate vecmath;
extern crate typemap;
extern crate image;
extern crate freetype;
extern crate color as ext_color;
extern crate glyph_packer;
extern crate num;

use std::error::Error;
use std::io::Error as IoError;
use std::convert::From;

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

pub use glium::index::PrimitiveType;
pub use glium::index::PrimitiveType::*;
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
            &LuxError::WindowError(ref s) => &s[..],
            &LuxError::OpenGlError(ref s) => &s[..],
            &LuxError::ShaderError(ref e) => e.description(),
            &LuxError::FontError(_, ref s) => &s[..],
            &LuxError::IoError(ref ioe) => Error::description(ioe),
            &LuxError::FontNotLoaded(ref s) => &s[..],
        }
    }
}

impl From<FreetypeError> for LuxError {
    fn from(e: FreetypeError) -> LuxError {
        use std::fmt::Write;
        let mut bf = String::new();
        write!(&mut bf, "{}", e).unwrap();
        LuxError::FontError(e, bf)
    }
}

impl From<glium::ProgramCreationError> for LuxError {
    fn from(e: glium::ProgramCreationError) -> LuxError {
        LuxError::ShaderError(e)
    }
}

impl From<IoError> for LuxError {
    fn from(ioe: IoError) -> LuxError {
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
