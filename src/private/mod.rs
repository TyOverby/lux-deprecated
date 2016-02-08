pub mod interactive;
pub mod canvas;
pub mod raw;
pub mod gfx_integration;
pub mod glutin_window;
pub mod color;
pub mod sprite;
//pub mod font;
pub mod error;
pub mod colors;
pub mod game;
pub mod accessors;
pub mod primitive_canvas;
pub mod shaders;
pub mod types;

pub mod font {
    pub struct FontCache;
    use std::collections::HashMap;
    use std::convert::{Into, AsRef};

    use super::types::Float;

    use vecmath;
    use super::accessors::{HasDisplay, HasFontCache};
    use super::color::Color;
    use super::error::{LuxError, LuxResult};
    use super::raw::{Colored, Transform};
    use super::canvas::Canvas;
    use super::sprite::{Sprite, IntoSprite};

    /// A context that contains information about the text that can be drawn to the screen.
    #[must_use = "text references just contains a drawing context, and must be drawn with `draw()`"]
    pub struct ContainedText<'a, C: 'a + HasDisplay + HasFontCache + Canvas, S: 'a + AsRef<str>> {
        canvas: &'a mut C,
        text: S,
        pos: (Float, Float),
        transform: [[Float; 4]; 4],
        color: [Float; 4],

        size: u16,
        font_family: String

    }

    /// Any struct that implements `TextDraw` can draw text to it.
    ///
    /// The only known implementation of `TextDraw` is Frame.
    pub trait TextDraw: TextLoad + Canvas {
        /// Starts drawing some text at a position.
        ///
        /// Text size and text font can be configured on the returned `ContainedText`
        /// object and finally drawn to the canvas with `.draw()`.
        fn text<'a, S: 'a + AsRef<str>>(&'a mut self, text: S, x: Float, y: Float) -> ContainedText<'a, Self, S> {
            /*
             * let (font_fam, size) = self.font_cache().current.clone().unwrap_or_else(|| ("SourceCodePro".to_string(), 20));
             */
            let (font_fam, size) = ("SourceCodePro".to_string(), 20);

            ContainedText {
                canvas: self,
                text: text,
                pos: (x, y),
                size: size,
                font_family: font_fam,
                transform: vecmath::mat4_id(),
                color: [0.0, 0.0, 0.0, 1.0]
            }
        }

    }

    /// Any struct that implements TextLoad can have fonts atlases added
    /// to an internal cache.
    ///
    /// Fonts must be loaded before they can be drawn with `TextDraw`.
    pub trait TextLoad: Sized + HasDisplay + HasFontCache {
        /// Adds a rendered font to the font cache.
        fn cache<F: IntoSprite>(&mut self, name: &str, size: u16, rendered: ()) -> LuxResult<()> {
            /*
            let rendered = rendered.map(|i| i.into_sprite(self.borrow_display()))
                                   .reskin();
            self.font_cache().cache(name, size, try!(rendered));
            */
            Ok(())
        }

        /// Removes a rendered font from the cache.
        fn clear(&mut self, name: &str, size: u16) {
            /*self.font_cache().clear(name, size);*/
        }

        /// Sets a font as the current font.
        fn use_font(&mut self, name: &str, size: u16) -> LuxResult<()> {
            /*
            self.font_cache().use_font(name, size).map(|_| ())
            */
            Ok(())
        }

    }

    impl <T> TextLoad for T where T: Sized + HasDisplay + HasFontCache { }
    impl <T> TextDraw for T where T: TextLoad + Canvas {  }
}

pub mod constants {
    #[cfg(feature="freetype")]
    pub static SOURCE_CODE_PRO_REGULAR: &'static[u8] =
        include_bytes!("../../resources/SourceCodePro-Regular.ttf");

    // Rendered images
    pub static SCP_12_PNG: &'static[u8] =
        include_bytes!("../../resources/SourceCodePro-Regular-12.png");
    pub static SCP_20_PNG: &'static[u8] =
        include_bytes!("../../resources/SourceCodePro-Regular-20.png");
    pub static SCP_30_PNG: &'static[u8] =
        include_bytes!("../../resources/SourceCodePro-Regular-30.png");

    // Info files
    pub static SCP_12_BINCODE: &'static[u8] =
        include_bytes!("../../resources/SourceCodePro-Regular-12.bincode");
    pub static SCP_20_BINCODE: &'static[u8] =
        include_bytes!("../../resources/SourceCodePro-Regular-20.bincode");
    pub static SCP_30_BINCODE: &'static[u8] =
        include_bytes!("../../resources/SourceCodePro-Regular-30.bincode");
}
