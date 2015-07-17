extern crate rustc_serialize;

use std::collections::HashMap;

/// Placement information about a specific character.
#[derive(Clone, Copy, RustcEncodable, RustcDecodable)]
pub struct CharInfo {
    /// The position in the image of the char
    pub image_position: (u32, u32),
    /// The (width, height) of the rendered character in the image
    pub image_size: (u32, u32),

    /// The number of pixels (x, y) that are advanced after this character
    /// is drawn.
    pub advance: (i32, i32),
    /// The distance that the pen should move before printing the character.
    pub pixel_offset: (i32, i32),
}

/// A representation of a fully-rendered font that contains a atlas image
/// and the metadata required to draw from it.
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct RenderedFont<I> {
    family_name: Option<String>,
    style_name: Option<String>,

    image: I,
    line_height: u32,
    max_width: u32,
    char_info: HashMap<char, CharInfo>,
    kerning: HashMap<(char, char), (i32, i32)>
}

/// The position of a character when drawn from a string.
pub struct OutputPosition {
    /// The character being drawn
    pub c: char,
    /// The position of the character on the screen
    pub screen_pos: (i32, i32),

    /// More information about the drawn character
    pub char_info: CharInfo
}

impl <I> RenderedFont<I> {
    pub fn new(
        family_name: Option<String>,
        style_name: Option<String>,

        image: I,
        line_height: u32,
        max_width: u32,
        char_info: HashMap<char, CharInfo>,
        kerning: HashMap<(char, char), (i32, i32)>) -> RenderedFont<I> {

        RenderedFont {
            family_name: family_name,
            style_name: style_name,

            image: image,
            line_height: line_height,
            max_width: max_width,
            char_info: char_info,
            kerning: kerning
        }
    }

    /// Returns the offsets `(dx, dy)` in pixels that should be applied
    /// to the difference in position between chars `a` and `b` where
    /// `a` comes immediately before `b` in the text.
    ///
    /// If the font doesn't specify a special kerning between these
    /// characters, `(0, 0)` is returned instead.
    pub fn kerning(&self, a: char, b: char) -> (i32, i32) {
        self.kerning.get(&(a, b)).cloned().unwrap_or((0, 0))
    }

    /// Returns the suggested distance between lines of text.
    pub fn line_height(&self) -> u32 {
        self.line_height
    }

    /// Returns the maximum width of a single char using this font.
    pub fn max_width(&self) -> u32 {
        self.max_width
    }

    /// Returns the offset and advance information regarding the specified
    /// character.
    pub fn char_info(&self, c: char) -> Option<CharInfo> {
        self.char_info.get(&c).cloned()
    }

    /// Returns the name of this font family e.g. Times New Roman
    pub fn family_name(&self) -> Option<&str> {
        self.family_name.as_ref().map(|a| &a[..])
    }

    /// Returns the name of the style e.g. (Bold).
    pub fn style_name(&self) -> Option<&str> {
        self.style_name.as_ref().map(|a| &a[..])
    }

    /// Returns a reference to the contained image.
    pub fn image(&self) -> &I {
        &self.image
    }

    /// Returns a mutable reference to the contained image.
    pub fn image_mut(&mut self) -> &mut I {
        &mut self.image
    }

    /// Applies a transformation function to the image of this rendered font
    /// producing a new rendered font with that image.
    pub fn map_img<A, B, F>(self, mapping_fn: F) -> (RenderedFont<A>, B)
    where F: FnOnce(I) -> (A, B) {
        let (r, e) = mapping_fn(self.image);
        (RenderedFont {
            family_name: self.family_name,
            style_name: self.style_name,

            image: r,
            line_height: self.line_height,
            max_width: self.max_width,
            char_info: self.char_info,
            kerning: self.kerning
        }, e)

    }

    /// Given a string, this function returns a vec containing all of the
    /// positions of each character as it should be rendered to the screen.
    ///
    /// The position is relative to the (0, 0) coordinate, and progress
    /// in the +x, +y direction.
    pub fn positions_for(&self, text: &str) -> Vec<OutputPosition> {
        let mut out = Vec::with_capacity(text.len());

        let mut x: i32 = 0;
        let mut y: i32 = self.line_height() as i32;

        let mut prev = None;

        for current in text.chars() {
            if current == '\n' {
                x = 0;
                y += self.line_height() as i32;
                prev = None;
                continue;
            }

            if let Some(prev) = prev {
                let (dx, dy) = self.kerning(prev, current);
                x += dx;
                y += dy;
            }

            if let Some(char_info) = self.char_info(current) {
                let (ox, oy) = char_info.pixel_offset;
                let pos = (x + ox as i32, y - oy as i32);

                out.push(OutputPosition {
                    c: current,
                    screen_pos: pos,
                    char_info: char_info
                });

                let (dx, dy) = char_info.advance;
                x += dx as i32;
                y += dy as i32;
            }

            prev = Some(current);
        }

        out
    }
}

