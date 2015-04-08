#![feature(plugin, unboxed_closures, unsafe_destructor, collections)]
#![feature(slice_patterns, debug_builders)]

#[macro_use]
extern crate glium;

extern crate glutin;
extern crate vecmath;
extern crate typemap;
extern crate image;
extern crate freetype;
extern crate color as ext_color;
extern crate glyph_packer;
extern crate num;
extern crate time;

pub mod interactive;
pub mod figure;
pub mod canvas;
pub mod raw;
pub mod gfx_integration;
pub mod glutin_window;
pub mod color;
pub mod sprite;
pub mod font;
pub mod error;
pub mod colors;
pub mod extend;
pub mod loader;
pub mod game;

pub mod prelude {
    pub use ::gfx_integration::{ColorVertex, TexVertex};
    pub use ::canvas::{LuxCanvas, PrimitiveCanvas, Ellipse, Rectangle, ContainedSprite};
    pub use ::error::{LuxError, ImageError, FreetypeError, LuxResult};
    pub use ::interactive::*;
    pub use ::interactive::Event::*;
    pub use ::interactive::MouseButton::*;
    pub use ::raw::{Colored, StackedColored, Transform, StackedTransform};
    pub use ::glutin_window::{Window, Frame};
    pub use ::color::{Color, rgb, rgba, hsv, hsva, hex_rgb, hex_rgba};
    pub use ::sprite::{Sprite, SpriteLoader, NonUniformSpriteSheet, UniformSpriteSheet};
    pub use ::figure::Figure;
    pub use ::font::{FontCache, TextDraw, FontLoad, gen_sheet};

    pub use glium::index::PrimitiveType;
    pub use glium::index::PrimitiveType::*;
    pub use super::extend::LuxExtend;
    pub use colors;
}
