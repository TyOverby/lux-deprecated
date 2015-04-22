#![allow(unused)]

use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::path::Path;
use std::io::Result as IoResult;
use std::fs::File;
use std::convert::{Into, AsRef};
use std::cell::{RefMut, Ref};

use image;
use freetype;
use glium;
use glyph_packer;
use vecmath;
use lux_constants::*;

use super::accessors::{HasDisplay, HasFontCache};
use super::prelude::{
    Color,
    LuxError,
    Colored,
    Transform,
    LuxCanvas,
    Sprite,
    TextureLoader,
    TexVertex,
    NonUniformSpriteSheet,
    LuxResult
};

pub type FontSheet = NonUniformSpriteSheet<char>;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[doc(hidden)]
pub struct CharOffset {
    advance: (i64, i64),
    bitmap_offset: (i64, i64)
}

#[doc(hidden)]
pub struct FontCache {
    library: freetype::Library,
    faces: HashMap<String, freetype::Face<'static>>,
    rendered: HashMap<(String, u32), Rc<RenderedFont>>,
}

#[doc(hidden)]
pub struct RenderedFont {
    pub name: String,
    pub size: u32,
    pub font_sheet: NonUniformSpriteSheet<char>,
    pub offsets: HashMap<char, CharOffset>
}

#[must_use = "text references contain context, and must be drawn with `draw()`"]
pub struct ContainedText<'a, C: 'a + HasDisplay + HasFontCache + LuxCanvas, S: 'a + AsRef<str>> {
    canvas: &'a mut C,
    text: S,
    pos: (f32, f32),
    transform: [[f32; 4]; 4],
    color: [f32; 4],

    size: u16,
    font_family: String

}

pub trait FontLoad {
    fn load_font<P: AsRef<Path>>(&mut self, name: &str, path: &P) -> LuxResult<()>;
    fn preload_font(&mut self, name: &str, size: u32) -> LuxResult<()>;
}

pub trait TextDraw: Sized + LuxCanvas + HasDisplay + HasFontCache {
    fn text<'a, S: 'a + AsRef<str>>(&'a mut self, text: S, x: f32, y : f32) -> ContainedText<'a, Self, S> {
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

impl <T> TextDraw for T where T: Sized + LuxCanvas + HasDisplay + HasFontCache {
}

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
        let window_c: glium::Display = self.canvas.clone_display();

        let rendered = {
            let mut font_cache = self.canvas.font_cache();
            let mut font_cache = font_cache.as_mut().unwrap();
            let rendered = try!(font_cache.use_font(&mut |img: image::DynamicImage| {
                let img = glium::texture::Texture2d::new(&window_c, img.flipv());
                Sprite::new(Rc::new(img))
            }, &self.font_family[..], self.size as u32));
            rendered.clone()
        };

        let mut font_cache = self.canvas.font_cache().take().unwrap();
        let render_result = {
            let face = font_cache.faces.get_mut(&self.font_family[..]).unwrap();
            let (x, y) = self.pos;
            rendered.render_string(self.canvas, self.text.as_ref(), face, x, y, self.color)
        };
        *self.canvas.font_cache() = Some(font_cache);
        render_result
    }

    /// Returns the height of one line of text with the selected font.
    pub fn line_height(&mut self) -> Option<i64> {
        let mut font_cache = self.canvas.font_cache();
        let mut font_cache = font_cache.as_mut().unwrap();
        let face = font_cache.faces.get_mut(&self.font_family);
        face.and_then(|face| face.size_metrics().map(|m| m.height / 64))
    }

    /// Returns an iterator containing each character in the input text along
    /// with the position and the size.
    ///
    /// These positions are absolute, and are not relative to the position that
    /// the text will be drawn on the screen.
    ///
    /// (char, position, size)
    pub fn absolute_positions(&mut self) -> LuxResult<Vec<(char, (i64, i64), (i64, i64))>> {
        let window_c: glium::Display = self.canvas.clone_display();

        let rendered = {
            let mut font_cache = self.canvas.font_cache();
            let mut font_cache = font_cache.as_mut().unwrap();
            let rendered = try!(font_cache.use_font(&mut |img: image::DynamicImage| {
                let img = glium::texture::Texture2d::new(&window_c, img.flipv());
                Sprite::new(Rc::new(img))
            }, &self.font_family[..], self.size as u32));
            rendered.clone()
        };

        let mut font_cache = self.canvas.font_cache().take().unwrap();
        let mut positions_result = {
            let face = font_cache.faces.get_mut(&self.font_family[..]).unwrap();
            let (x, y) = self.pos;
            rendered.positions(self.text.as_ref(), face)
        };
        *self.canvas.font_cache() = Some(font_cache);
        positions_result
    }

    pub fn positions(&mut self) -> LuxResult<Vec<(char, (f32, f32), (f32, f32))>> {
        self.absolute_positions().map(|poses| {
            poses.into_iter().map(
                |(c, (px, py), (sx, sy))|
                    (c,
                     (px as f32 + self.pos.0, py as f32 + self.pos.1),
                     (sx as f32, sy as f32))
              ).collect()
        })
    }

    /// Returns the bounding box around this text.
    ///
    /// ((start_x, start_y), (width, height))
    pub fn bounding_box(&mut self) -> LuxResult<((f32, f32), (f32, f32))> {
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
    fn current_matrix(&self) -> &[[f32; 4]; 4] {
        &self.transform
    }

    fn current_matrix_mut(&mut self) -> &mut [[f32; 4]; 4] {
        &mut self.transform
    }
}

impl <'a, A, B: AsRef<str>> Colored for ContainedText<'a, A, B>
where A: HasDisplay + HasFontCache + LuxCanvas {
    fn color(&self) -> [f32; 4] {
        self.color
    }

    fn set_color<C: Color>(&mut self, color: C) -> &mut Self {
        self.color = color.to_rgba();
        self
    }
}

impl <T> FontLoad for T where T: HasDisplay + HasFontCache {
    fn load_font<P: AsRef<Path> + ?Sized>(&mut self, name: &str, path: &P) -> LuxResult<()> {
        let mut font_cache: RefMut<Option<FontCache>> = self.font_cache();
        font_cache.as_mut().unwrap().load(name, path.as_ref())
    }

    fn preload_font(&mut self, name: &str, size: u32) -> LuxResult<()> {
        use std::fs::File;
        let window_c: glium::Display = self.clone_display();

        let mut font_cache = self.font_cache();
        let mut font_cache = font_cache.as_mut().unwrap();
        let res = font_cache.use_font(&mut |img: image::DynamicImage| {
            let img = img.flipv();
            let tex = glium::texture::Texture2d::new(&window_c, img);
            Sprite::new(Rc::new(tex))
        }, name, size);
        res.map(|_|())
    }
}

impl FontCache {
    pub fn new<S>(mut loader: S) -> LuxResult<FontCache> where S: FnMut(image::DynamicImage) -> Sprite {
        // Load the default font.
        let lib = try!(freetype::Library::init());

        let mut fc = FontCache {
            library: lib,
            faces: HashMap::new(),
            rendered: HashMap::new(),
        };

        fc.load_bytes("SourceCodePro", SOURCE_CODE_PRO_REGULAR);
        fc.use_font(&mut loader, "SourceCodePro", 20);

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

    // TODO: rewrite the body of this method with entry()
    pub fn use_font<S>(&mut self, loader: &mut S, name: &str, size: u32) -> LuxResult<&Rc<RenderedFont>>
    where S: FnMut(image::DynamicImage) -> Sprite {
        use std::fmt::Write;

        let key = (name.to_string(), size);

        {
            if self.rendered.contains_key(&key) {
                return Ok(self.rendered.get(&key).unwrap())
            }
        }

        {
            if let Some(face) = self.faces.get_mut(name) {
                let sheet = RenderedFont::new(loader, face, name.to_string(), size);
                let sheet = Rc::new(try!(sheet));
                self.rendered.insert(key.clone(), sheet.clone());
                return Ok(self.rendered.get(&key).unwrap())
            } else {
                let err = format!("Font not loaded: {}", name);
                return Err(LuxError::FontNotLoaded(err))
            }
        }
    }
}

impl RenderedFont {
    fn new<S>(loader: &mut S, face: &mut freetype::Face, name: String, size: u32)
    -> LuxResult<RenderedFont> where S: FnMut(image::DynamicImage) -> Sprite {
        let (sheet, offsets) = try!(gen_sheet(loader, face, size));
        Ok(RenderedFont {
            name: name,
            size: size,
            font_sheet: sheet,
            offsets: offsets
        })
    }

    fn positions(&self, text: &str, face: &mut freetype::Face) -> LuxResult<Vec<(char, (i64, i64), (i64, i64))>> {
        let sheet = &self.font_sheet;
        let mut out = Vec::with_capacity(text.len());

        try!(face.set_pixel_sizes(0, self.size));

        let mut prev: Option<char> = None;
        let height_offset = face.size_metrics().map(|m| m.height / 64).unwrap_or(0);

        let mut x = 0;
        let mut y = height_offset;
        for current in text.chars() {
            if current == '\n' {
                x = 0;
                y += height_offset;
                prev = None;
                continue;
            }

            if let Some(prev) = prev {
                let delta = face.get_kerning(
                        face.get_char_index(prev as usize),
                        face.get_char_index(current as usize),
                        freetype::face::KerningMode::KerningDefault);
                let delta = try!(delta);
                x += delta.x / 64;
            }

            let offset = self.offsets.get(&current).cloned().unwrap_or(CharOffset {
                advance: (0, 0),
                bitmap_offset: (0, 0)
            });

            let pos = (x + offset.bitmap_offset.0, y - offset.bitmap_offset.1);
            let size = ((offset.advance.0 / 64), (offset.advance.1 / 64));

            out.push((current, pos, size));

            x += offset.advance.0 / 64;
            y += offset.advance.1 / 64;

            prev = Some(current);
        }
        Ok((out))
    }

    fn render_string<C>(&self, canvas: &mut C, text: &str,
                        face: &mut freetype::Face, x: f32, y: f32, color: [f32; 4])
    -> LuxResult<()> where C: LuxCanvas {
        let sheet = &self.font_sheet;
        for (chr, (ox, oy), (_, _)) in try!(self.positions(text, face)) {
            canvas.sprite(&sheet.get(&chr), x as f32 + ox as f32, y as f32 + oy as f32).set_color(color).draw();
        }

        Ok(())
    }
}

fn gen_sheet<S>(loader: &mut S, face: &mut freetype::Face, size: u32)
-> LuxResult<(FontSheet, HashMap<char, CharOffset>)> where S: FnMut(image::DynamicImage) -> Sprite {
    try!(face.set_pixel_sizes(0, size));

    let mut v = vec![];
    for i in 1u8 .. 255 {
        v.push(i as char);
    }

    let (texture, map) = merge_all(v.into_iter().map(|c| (c, char_to_img(face, c))));
    let sprite = loader(texture);
    let mut sprite = sprite.as_nonuniform_sprite_sheet();
    let mut offsets = HashMap::new();
    for (k, (r, offset)) in map {
        sprite.associate(k, (r.x, r.y), (r.w, r.h));
        offsets.insert(k, offset);
    }

    Ok((sprite, offsets))
}

fn char_to_img(face: &freetype::Face, c: char) -> LuxResult<(image::DynamicImage, CharOffset)> {
    fn buf_to_vec(bf: &[u8], width: u32, height: u32) -> image::DynamicImage {
        let mut v = Vec::with_capacity((width * height * 2) as usize);
        for &p in bf {
            v.push(p);
            v.push(p);
        }
        image::DynamicImage::ImageLumaA8(
            image::ImageBuffer::from_vec(width, height, v).unwrap())
    }

    try!(face.load_char(c as usize, freetype::face::RENDER));

    let glyph = face.glyph();
    let bit = glyph.bitmap();

    let advance = glyph.advance();
    let advance = (advance.x, advance.y);
    let offset = (glyph.bitmap_left() as i64, glyph.bitmap_top() as i64);
    let char_offsets = CharOffset {
        advance: advance,
        bitmap_offset: offset
    };

    Ok((buf_to_vec(bit.buffer(), bit.width() as u32, bit.rows() as u32), char_offsets))
}

fn merge_all<A: ::std::fmt::Debug, I>(mut images: I) ->
(image::DynamicImage, HashMap<A, (glyph_packer::Rect, CharOffset)>)
where I: Iterator<Item=(A, LuxResult<(image::DynamicImage, CharOffset)>)>,
      A: Eq + Hash {
    use glyph_packer::{Packer, GrowingPacker};
    use std::mem::replace;

    let mut size = 256u32;
    let mut packer: glyph_packer::SkylinePacker<_> = {
        let bf = image::DynamicImage::new_luma_a8(size, size);
        Packer::new(bf)
    };

    let mut mapping = HashMap::new();
    packer.set_margin(5);

    for (a, comp) in images {
        match comp {
            Ok((img, adv)) => {
                let rect = packer.pack_resize(&img, |(x, y)| (x * 2, y * 2));
                mapping.insert(a, (rect, adv));
            }
            Err(e) => { }
        }
    }

    let buf = packer.into_buf();
    (buf, mapping)
}
