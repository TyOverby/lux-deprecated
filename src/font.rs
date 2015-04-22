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
use lux_constants::*;

use super::accessors::{HasDisplay, HasFontCache};
use super::prelude::{
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
    pub current: Option<Rc<RenderedFont>>
}

#[doc(hidden)]
pub struct RenderedFont {
    pub name: String,
    pub size: u32,
    pub font_sheet: NonUniformSpriteSheet<char>,
    pub offsets: HashMap<char, CharOffset>
}

pub struct ContainedText<'a, C: 'a, S: 'a + AsRef<str>> {
    canvas: &'a mut C,
    text: &'a S,
    pos: (f32, f32),
    transform: [[f32; 4]; 4],
    color: [f32; 4]
}

pub trait FontLoad {
    fn load_font<P: AsRef<Path>>(&mut self, name: &str, path: &P) -> LuxResult<()>;
    fn preload_font(&mut self, name: &str, size: u32) -> LuxResult<()>;
}

pub trait TextDraw {
    fn draw_text(&mut self, text: &str, x: f32, y: f32) -> LuxResult<()>;
    fn set_font(&mut self, name: &str, size: u32) -> LuxResult<()>;
}

pub trait TextDraw2 {

}

impl <'a, A, B: AsRef<str>> Transform for ContainedText<'a, A, B> {
    fn current_matrix(&self) -> &[[f32; 4]; 4] {
        &self.transform
    }

    fn current_matrix_mut(&mut self) -> &mut [[f32; 4]; 4] {
        &mut self.transform
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
        res
    }
}

impl <T> TextDraw for T where T: HasDisplay + HasFontCache + LuxCanvas {
    fn draw_text(&mut self, text: &str, x: f32, y: f32) -> LuxResult<()> {
        let c =  self.color();

        // Take the font cache, then put it back when we're done.
        let mut font_cache = self.font_cache().take().unwrap();
        try!(font_cache.draw_onto(self, text, x, y, c));
        *self.font_cache() = Some(font_cache);
        Ok(())
    }

    fn set_font(&mut self, name: &str, size: u32) -> LuxResult<()> {
        use std::fs::File;

        let window_c: glium::Display = self.clone_display();

        let mut font_cache = self.font_cache();
        let mut font_cache = font_cache.as_mut().unwrap();
        let res = font_cache.use_font(&mut |img: image::DynamicImage| {
            let img = img.flipv();

            let mut out_path = File::create("out.png").unwrap();
            let _ = img.save(&mut out_path, ::image::ImageFormat::PNG).unwrap();


            let img = glium::texture::Texture2d::new(&window_c, img);
            Sprite::new(Rc::new(img))
        }, name, size);
        res
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
            current: None
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

    pub fn use_font<S>(&mut self, loader: &mut S, name: &str, size: u32) -> LuxResult<()>
    where S: FnMut(image::DynamicImage) -> Sprite {
        use std::fmt::Write;

        let key = (name.to_string(), size);

        // If we are already set, then this is a no-op.
        if let &Some(ref font_sheet) = &self.current {
            if font_sheet.name == name && font_sheet.size == size {
                return Ok(());
            }
        }

        if let Some(font_sheet) = self.rendered.get(&key) {
            self.current = Some(font_sheet.clone());
            return Ok(());
        }

        if let Some(face) = self.faces.get_mut(name) {
            let sheet = RenderedFont::new(loader, face, name.to_string(), size);
            let sheet = Rc::new(try!(sheet));
            self.rendered.insert(key, sheet.clone());
            self.current = Some(sheet);
        } else {
            let err = format!("Font not loaded: {}", name);
            return Err(LuxError::FontNotLoaded(err))
        }

        return Ok(());
    }

    pub fn draw_onto<S>(&mut self, canvas: &mut S, text: &str,
                        x: f32, y: f32, color: [f32; 4]) -> LuxResult<()>
    where S: LuxCanvas {
        let face = self.faces.get_mut(&self.current.as_ref().unwrap().name).unwrap();
        self.current.as_ref().unwrap().render_string(canvas, text, face, x, y, color)
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

    fn render_string<C>(&self, canvas: &mut C, text: &str,
                        face: &mut freetype::Face, x: f32, y: f32, color: [f32; 4])
    -> LuxResult<()> where C: LuxCanvas {
        let sheet = &self.font_sheet;
        let original_x = x;
        let original_y = y;

        try!(face.set_pixel_sizes(0, self.size));

        let mut prev: Option<char> = None;
        let height_offset = face.size_metrics().map(|m| m.height / 64).unwrap_or(0);

        let mut x = x;
        let mut y = y + height_offset as f32;
        for current in text.chars() {
            if current == '\n' {
                x = original_x;
                y += height_offset as f32;
                prev = None;
                continue;
            }

            if let Some(prev) = prev {
                let delta = face.get_kerning(
                        face.get_char_index(prev as usize),
                        face.get_char_index(current as usize),
                        freetype::face::KerningMode::KerningDefault);
                let delta = try!(delta);
                x += (delta.x / 64) as f32;
            }

            let offset = self.offsets.get(&current).cloned().unwrap_or(CharOffset {
                advance: (0, 0),
                bitmap_offset: (0, 0)
            });
            canvas.sprite(&sheet.get(&current),
                          x + offset.bitmap_offset.0 as f32,
                          y - offset.bitmap_offset.1 as f32).set_color(color).draw();

            x += (offset.advance.0 / 64) as f32;
            y += (offset.advance.1 / 64) as f32;

            prev = Some(current);
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
