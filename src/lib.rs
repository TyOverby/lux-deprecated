#![warn(missing_docs)]

#[macro_use]
extern crate glium;

extern crate glutin;
extern crate vecmath;
extern crate typemap;
extern crate image;
extern crate freetype;
extern crate num;
extern crate clock_ticks;
extern crate lux_constants;
extern crate fontcache;
extern crate freetype_atlas;
extern crate reuse_cache;

mod private;

pub use private::types;

pub use private::error::{LuxError, LuxResult};

pub mod color {
    pub use private::color::{Color, rgb, rgba, hsv, hsva};
    pub use private::colors::*;
}

pub mod graphics {
    pub use private::canvas::{LuxCanvas, Rectangle, Ellipse, ContainedSprite};
    pub use private::primitive_canvas::PrimitiveCanvas;
    pub use private::sprite::{
        Sprite,
        Texture,
        DrawableTexture,
        UniformSpriteSheet,
        NonUniformSpriteSheet,
        TextureLoader
    };
}

pub mod interactive {
    pub use private::interactive::{
        keycodes,
        EventIterator,
        Event,
        MouseButton,
        Interactive,
        AbstractKey
    };
}

pub mod window {
    pub use private::glutin_window::{Window, Frame};
}

pub mod modifiers {
    pub use private::raw::{Colored, Transform};
}

pub mod game {
    pub use private::game::{Game, GameRunner};
}

pub mod font {
    pub use private::font::{ContainedText, FontLoad, TextDraw};
}

pub mod prelude {
    pub use color::{Color, rgb, rgba, hsv, hsva};
    pub use graphics::LuxCanvas;
    pub use interactive::Interactive;
    pub use window::{Window, Frame};
    pub use interactive::EventIterator;
    pub use modifiers::{Colored, Transform};
    pub use font::{FontLoad, TextDraw};

    pub use LuxError;
    pub use LuxResult;
}
