use std::collections::HashMap;
use std::convert::{Into, AsRef};

use super::types::Float;

use vecmath;
use fontcache;

pub use fontcache::OutputPosition;

use super::accessors::{HasDisplay, HasFontCache};
use super::color::Color;
use super::error::{LuxError, LuxResult};
use super::raw::{Colored, Transform};
use super::canvas::Canvas;
use super::sprite::{Sprite, IntoSprite};

pub struct FontCache {
    rendered: HashMap<(String, u16), fontcache::RenderedFont<Sprite>>,
    current: Option<(String, u16)>
}

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
pub trait TextDraw: Sized + Canvas + HasDisplay + HasFontCache {
    /// Starts drawing some text at a position.
    ///
    /// Text size and text font can be configured on the returned `ContainedText`
    /// object and finally drawn to the canvas with `.draw()`.
    fn text<'a, S: 'a + AsRef<str>>(&'a mut self, text: S, x: Float, y: Float) -> ContainedText<'a, Self, S> {
        let (font_fam, size) = self.font_cache().current.clone().unwrap_or_else(|| ("SourceCodePro".to_string(), 20));

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

    /// Adds a rendered font to the font cache.
    fn cache<F: IntoSprite>(&mut self, name: &str, size: u16, rendered: fontcache::RenderedFont<F>) -> LuxResult<()> {
        let rendered = rendered.map(|i| i.into_sprite(self.borrow_display()))
                               .reskin();
        self.font_cache().cache(name, size, try!(rendered));
        Ok(())
    }

    /// Removes a rendered font from the cache.
    fn clear(&mut self, name: &str, size: u16) {
        self.font_cache().clear(name, size);
    }

    /// Sets a font as the current font.
    fn use_font(&mut self, name: &str, size: u16) -> LuxResult<()> {
        self.font_cache().use_font(name, size).map(|_| ())
    }
}

impl <T> TextDraw for T where T: Sized + Canvas + HasDisplay + HasFontCache { }

impl <'a, C: 'a + HasDisplay + HasFontCache + Canvas, S: 'a + AsRef<str>> ContainedText<'a, C, S> {
    /// Sets the size of the font.
    pub fn size(&mut self, size: u16) -> &mut ContainedText<'a, C, S> {
        self.size = size;
        self
    }

    /// Sets the font to be used.
    pub fn font<A: Into<String>>(&mut self, font_family: A) -> &mut ContainedText<'a, C, S> {
        self.font_family = font_family.into();
        self
    }

    /// Draws the font to the screen.
    pub fn draw(&mut self) -> LuxResult<()> {
        let canvas: &mut C = {
            let x: &mut C = self.canvas;
            unsafe { ::std::mem::transmute(x) }
        };
        let positions = try!(self.absolute_positions());

        let mut fc = self.canvas.font_cache();
        let rendered = try!(fc.use_font(self.font_family.as_ref(), self.size));

        for OutputPosition{c: _, screen_pos: (x, y), char_info} in positions {
            let subsprite = rendered.image().sub_sprite(char_info.image_position,
                                                char_info.image_size);
            if let Some(sp) = subsprite.as_ref() {
                canvas.sprite(
                    sp,
                    x as Float + self.pos.0,
                    y as Float + self.pos.1).color(self.color).draw();
            }
        }
        Ok(())
    }

    /// Returns the height of one line of text with the selected font.
    pub fn line_height(&mut self) -> LuxResult<u32> {
        let mut fc = self.canvas.font_cache();
        let rendered = try!(fc.use_font(self.font_family.as_ref(), self.size));
        Ok(rendered.line_height())
    }

    /// Returns the maximum horizontal distance that a character can move the pen
    /// while drawing.
    pub fn max_advance(&mut self) -> LuxResult<u32> {
        let mut fc = self.canvas.font_cache();
        let rendered = try!(fc.use_font(self.font_family.as_ref(), self.size));
        Ok(rendered.max_width())
    }

    /// Returns an iterator containing each character in the input text along
    /// with the position and the size.
    ///
    /// These positions are absolute, and are not relative to the position that
    /// the text will be drawn on the screen. (they start at position (0, 0))
    pub fn absolute_positions(&mut self) -> LuxResult<Vec<OutputPosition>> {
        let mut fc = self.canvas.font_cache();
        let rendered = try!(fc.use_font(self.font_family.as_ref(), self.size));
        Ok(rendered.positions_for(self.text.as_ref()))
    }

    /// Returns an iterator containing each character in the input text along
    /// with the position and the size.
    ///
    /// These positions are relative to the providex (x, y) coordinates that
    /// the text will be drawn at.
    pub fn positions(&mut self) -> LuxResult<Vec<(char, (Float, Float), (Float, Float))>> {
        self.absolute_positions().map(|poses| {
            poses.into_iter().map(
                |OutputPosition{c, screen_pos: (px, py), char_info}|
                    (c,
                    (px as Float + self.pos.0, py as Float + self.pos.1),
                    (char_info.advance.0 as Float, char_info.advance.1 as Float))
              ).collect()
        })
    }

    /// Returns the bounding box around this text.
    ///
    /// `((start_x, start_y), (width, height))`
    ///
    /// `start_x` and `start_y` are oriented to the top-left of the screen.
    ///
    /// `width` and `height` are pointing down and to the right.
    pub fn bounding_box(&mut self) -> LuxResult<((Float, Float), (Float, Float))> {
        let start = self.pos;
        let end = try!(self.positions())
                  .pop()
                  .map(|(_, (px, py), (sx, sy))| (px + sx, py + sy))
                  .unwrap_or(start);
        Ok((start, end))
    }

    /// Returns the length in pixels of the rendered string.
    pub fn get_length(&mut self) -> LuxResult<u32> {
        self.positions().map(|mut positions| {
            positions.pop().map(|(_, (x, _), (w, _))| (x + w) as u32)
                     .unwrap_or(0)
        })
    }
}

impl <'a, A, B: AsRef<str>> Transform for ContainedText<'a, A, B>
where A: HasDisplay + HasFontCache + Canvas {
    fn current_matrix(&self) -> &[[Float; 4]; 4] {
        &self.transform
    }

    fn current_matrix_mut(&mut self) -> &mut [[Float; 4]; 4] {
        &mut self.transform
    }
}

impl <'a, A, B: AsRef<str>> Colored for ContainedText<'a, A, B>
where A: HasDisplay + HasFontCache + Canvas {
    fn get_color(&self) -> [Float; 4] {
        self.color
    }

    fn color<C: Color>(&mut self, color: C) -> &mut Self {
        self.color = color.to_rgba();
        self
    }
}

impl FontCache {
    pub fn new() -> LuxResult<FontCache> {
        let fc = FontCache {
            rendered: HashMap::new(),
            current: None
        };
        Ok(fc)
    }

    fn cache(&mut self, name: &str, size: u16, rendered: fontcache::RenderedFont<Sprite>) {
        self.rendered.insert((name.to_string(), size), rendered);
    }
    fn clear(&mut self, name: &str, size: u16) {
        self.rendered.remove(&(name.to_string(), size));
    }

    fn use_font<'a>(&'a mut self, name: &str, size: u16) -> LuxResult<&'a fontcache::RenderedFont<Sprite>> {
        let tup = (name.to_string(), size);
        let res = self.rendered.get(&tup).ok_or_else(|| LuxError::FontNotLoaded(name.to_string()));
        self.current = Some(tup);
        res
    }
}
