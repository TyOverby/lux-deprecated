use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::path::Path;
use std::convert::{Into, AsRef};

use super::types::Float;

use freetype;
use glium;
use vecmath;
use fontcache;
use freetype_atlas;
use lux_constants::*;

pub use fontcache::OutputPosition;

use super::accessors::{HasDisplay, HasFontCache};
use super::prelude::{
    Color,
    LuxError,
    Colored,
    Transform,
    LuxCanvas,
    TextureLoader,
    LuxResult,
    Sprite,
};

#[doc(hidden)]
pub struct FontCache {
    library: freetype::Library,
    faces: HashMap<String, freetype::Face<'static>>,
    rendered: HashMap<(String, u32), fontcache::RenderedFont<Sprite>>,
}

#[must_use = "text references just contains context, and must be drawn with `draw()`"]
pub struct ContainedText<'a, C: 'a + HasDisplay + HasFontCache + LuxCanvas, S: 'a + AsRef<str>> {
    canvas: &'a mut C,
    text: S,
    pos: (Float, Float),
    transform: [[Float; 4]; 4],
    color: [Float; 4],

    size: u16,
    font_family: String

}

pub trait FontLoad {
    fn load_font<P: AsRef<Path>>(&mut self, name: &str, path: &P) -> LuxResult<()>;
    fn preload_font(&mut self, name: &str, size: u32) -> LuxResult<()>;
}

pub trait TextDraw: Sized + LuxCanvas + HasDisplay + HasFontCache {
    fn text<'a, S: 'a + AsRef<str>>(&'a mut self, text: S, x: Float, y : Float) -> ContainedText<'a, Self, S> {
        ContainedText {
            canvas: self,
            text: text,
            pos: (x, y),
            size: 20,
            font_family: "SourceCodePro".to_string(),
            transform: vecmath::mat4_id(),
            color: [0.0, 0.0, 0.0, 1.0]
        }
    }
}

impl <T> TextDraw for T where T: Sized + LuxCanvas + HasDisplay + HasFontCache { }

impl <'a, C: 'a + HasDisplay + HasFontCache + LuxCanvas, S: 'a + AsRef<str>> ContainedText<'a, C, S> {
    pub fn size(&mut self, size: u16) -> &mut ContainedText<'a, C, S> {
        self.size = size;
        self
    }

    pub fn font<A: Into<String>>(&mut self, font_family: A) -> &mut ContainedText<'a, C, S> {
        self.font_family = font_family.into();
        self
    }

    pub fn draw(&mut self) -> LuxResult<()> {
        let canvas: &mut C = {
            let x: &mut C = self.canvas;
            unsafe { ::std::mem::transmute(x) }
        };
        let positions = try!(self.absolute_positions());

        let mut fc = self.canvas.font_cache();
        let d = self.canvas.borrow_display();
        let rendered = try!(fc.use_font(d, self.font_family.as_ref(), self.size as u32));

        for OutputPosition{c: _, screen_pos: (x, y), char_info} in positions {
            let subsprite = rendered.image().sub_sprite(char_info.image_position,
                                                char_info.image_size);
            if let Some(sp) = subsprite.as_ref() {
                canvas.sprite(
                        sp,
                        x as Float + self.pos.0,
                        y as Float + self.pos.1).set_color(self.color).draw()
            }
        }
        Ok(())
    }

    /// Returns the height of one line of text with the selected font.
    pub fn line_height(&mut self) -> LuxResult<u32> {
        let mut fc = self.canvas.font_cache();
        let d = self.canvas.borrow_display();
        let rendered = fc.use_font(d, self.font_family.as_ref(), self.size as u32);
        rendered.map(|rf| {
            rf.line_height()
        })
    }

    pub fn max_advance(&mut self) -> LuxResult<u32> {
        let mut fc = self.canvas.font_cache();
        let d = self.canvas.borrow_display();
        let rendered = fc.use_font( d, self.font_family.as_ref(), self.size as u32);
        rendered.map(|rf| {
            rf.max_width()
        })
    }

    /// Returns an iterator containing each character in the input text along
    /// with the position and the size.
    ///
    /// These positions are absolute, and are not relative to the position that
    /// the text will be drawn on the screen.
    ///
    /// (char, position, size)
    pub fn absolute_positions(&mut self) -> LuxResult<Vec<OutputPosition>> {
        let mut fc = self.canvas.font_cache();
        let d = self.canvas.borrow_display();
        let rendered = fc.use_font( d, self.font_family.as_ref(), self.size as u32);

        rendered.map(|rf| {
            rf.positions_for(self.text.as_ref())
        })
    }

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
where A: HasDisplay + HasFontCache + LuxCanvas {
    fn current_matrix(&self) -> &[[Float; 4]; 4] {
        &self.transform
    }

    fn current_matrix_mut(&mut self) -> &mut [[Float; 4]; 4] {
        &mut self.transform
    }
}

impl <'a, A, B: AsRef<str>> Colored for ContainedText<'a, A, B>
where A: HasDisplay + HasFontCache + LuxCanvas {
    fn color(&self) -> [Float; 4] {
        self.color
    }

    fn set_color<C: Color>(&mut self, color: C) -> &mut Self {
        self.color = color.to_rgba();
        self
    }
}

impl <T> FontLoad for T where T: HasDisplay + HasFontCache {
    fn load_font<P: AsRef<Path> + ?Sized>(&mut self, name: &str, path: &P) -> LuxResult<()> {
        self.font_cache().load(name, path.as_ref())
    }

    fn preload_font(&mut self, name: &str, size: u32) -> LuxResult<()> {
        let d = self.borrow_display();
        self.font_cache().use_font(d, name, size).map(|_| ())
    }
}

impl FontCache {
    pub fn new(loader: &glium::Display) -> LuxResult<FontCache> {
        // Load the default font.
        let lib = try!(freetype::Library::init());

        let mut fc = FontCache {
            library: lib,
            faces: HashMap::new(),
            rendered: HashMap::new(),
        };

        try!(fc.load_bytes("SourceCodePro", SOURCE_CODE_PRO_REGULAR));
        try!(fc.use_font(loader, "SourceCodePro", 20));

        Ok(fc)
    }

    pub fn load(&mut self, name: &str, path: &Path) -> LuxResult<()> {
        let face = try!(self.library.new_face(&path, 0));
        self.faces.insert(name.to_string(), face);
        Ok(())
    }

    pub fn load_bytes(&mut self, name: &str, bytes: &'static [u8]) -> LuxResult<()> {
        let face = try!(self.library.new_memory_face(&bytes[..], 0));
        self.faces.insert(name.to_string(), face);
        Ok(())
    }

    pub fn use_font<'a>(&'a mut self, display: &glium::Display, name: &str, size: u32)
    -> LuxResult<&'a fontcache::RenderedFont<Sprite>> {
        let mut v = vec![];
        for i in 0u8 .. 255 {
            v.push(i as char);
        }

        match self.rendered.entry((name.to_string(), size)) {
            Entry::Occupied(entry) => {
                Ok(entry.into_mut())
            }
            Entry::Vacant(entry) => {
                if let Some(face) = self.faces.get_mut(&name[..]) {
                    try!(face.set_pixel_sizes(0, size));
                    let rendered = try!(freetype_atlas::render(face, v.into_iter(), true));
                    let (sprited, _) = rendered.map_img(move |img|(
                       TextureLoader::texture_from_image(display, img).into_sprite(),()));
                    Ok(entry.insert(sprited))
                } else {
                    Err(LuxError::FontNotLoaded(name.to_string()))
                }
            }
        }
    }
}
