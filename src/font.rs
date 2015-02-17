#![allow(unused)]

use std::collections::hash_map::{HashMap, Hasher};
use std::hash::Hash;
use std::rc::Rc;
use std::old_path::Path;
use std::old_io::{File, IoResult};

use image;
use freetype;
use texture_packer;

use super::{
    LuxError,
    LuxCanvas,
    Sprite,
    SpriteLoader,
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
    faces: HashMap<String, freetype::Face>,
    rendered: HashMap<(String, u32), Rc<RenderedFont>>,
    pub current: Rc<RenderedFont>
}

#[doc(hidden)]
pub struct RenderedFont {
    pub name: String,
    pub size: u32,
    pub font_sheet: NonUniformSpriteSheet<char>,
    pub offsets: HashMap<char, CharOffset>
}

pub trait FontLoad {
    fn load_font(&mut self, name: &str, path: &Path) -> LuxResult<()>;
    fn preload_font(&mut self, name: &str, size: u32) -> LuxResult<()>;
}

pub trait TextDraw {
    fn draw_text(&mut self, text: &str, x: f32, y: f32) -> LuxResult<()>;
    fn set_font(&mut self, name: &str, size: u32) -> LuxResult<()>;
    fn get_font(&self) -> (String, u32);
}

pub trait TextDrawStack: TextDraw {
    fn with_font<F>(&mut self, name: &str, size: u32, f: F) -> LuxResult<()>
    where F: FnOnce(&mut Self) {
        let current = self.get_font();
        try!(self.set_font(name, size));
        f(self);
        try!(self.set_font(&current.0[], current.1));
        Ok(())
    }
}

impl <T: TextDraw> TextDrawStack for T {}

impl FontCache {
    pub fn new<S>(loader: S) -> LuxResult<FontCache> where S: FnOnce(image::DynamicImage) -> Sprite {
        // Load the default font.
        let lib = try!(freetype::Library::init());

        let mut fc = FontCache {
            library: lib,
            faces: HashMap::new(),
            rendered: HashMap::new(),
            current: unsafe {::std::mem::uninitialized() } //rendered.clone()
        };

        let bytes = include_bytes!("../resources/SourceCodePro-Regular.ttf");
        fc.load_bytes("SourceCodePro", bytes);
        fc.use_font(loader, "SourceCodePro", 16);

        Ok(fc)
    }

    pub fn load(&mut self, name: &str, path: &Path) -> LuxResult<()> {
        let bytes = try!(File::open(path).read_to_end());
        self.load_bytes(name, &bytes[])
    }

    pub fn load_bytes(&mut self, name: &str, bytes: &[u8]) -> LuxResult<()> {
        let face = try!(self.library.new_memory_face(&bytes[], 0));
        self.faces.insert(name.to_string(), face);
        Ok(())
    }

    pub fn use_font<S>(&mut self, loader: S, name: &str, size: u32) -> LuxResult<()>
    where S: FnOnce(image::DynamicImage) -> Sprite {
        use std::fmt::Writer;

        let key = (name.to_string(), size);
        if let Some(font_sheet) = self.rendered.get(&key) {
            self.current = font_sheet.clone();
            //println!("found sheet.");
            return Ok(());
        }

        if let Some(face) = self.faces.get_mut(name) {
            let sheet = RenderedFont::new(loader, face, name.to_string(), size);
            let sheet = Rc::new(try!(sheet));
            self.rendered.insert(key, sheet.clone());
            self.current = sheet;
            //println!("found facee made sheet.");
            return Ok(());
        }

        let mut bf = String::new();
        write!(&mut bf, "Font not loaded: {}", name).unwrap();

        Err(LuxError::FontNotLoaded(bf))
    }

    pub fn draw_onto<S>(&mut self, canvas: &mut S, text: &str,
                        x: f32, y: f32, color: [f32; 4]) -> LuxResult<()>
    where S: LuxCanvas {
        let face = self.faces.get_mut(&self.current.name).unwrap();
        self.current.render_string(canvas, text, face, x, y, color)
    }
}

impl RenderedFont {
    fn new<S>(loader: S, face: &mut freetype::Face, name: String, size: u32)
    -> LuxResult<RenderedFont> where S: FnOnce(image::DynamicImage) -> Sprite {
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

        face.set_pixel_sizes(0, self.size);

        let mut prev: Option<char> = None;
        let mut x = x + 0.5;
        let mut y = y + 0.5;
        for current in text.chars() {
            if let Some(prev) = prev {
                let delta = face.get_kerning(face.get_char_index(prev as usize),
                                             face.get_char_index(current as usize),
                                             freetype::face::KerningMode::KerningDefault);
                let delta = try!(delta);
                x += (delta.x >> 6) as f32;
            }

            let offset = self.offsets.get(&current).cloned().unwrap_or(CharOffset {
                advance: (0, 0),
                bitmap_offset: (0, 0)
            });
            canvas.sprite(&sheet.get(&current),
                          x + offset.bitmap_offset.0 as f32,
                          y - offset.bitmap_offset.1 as f32).color(color).draw();

            x += (offset.advance.0 >> 6) as f32;

            prev = Some(current);
        }
        Ok(())
    }
}

pub fn gen_sheet<S>(loader: S, face: &mut freetype::Face, size: u32)
-> LuxResult<(FontSheet, HashMap<char, CharOffset>)> where S: FnOnce(image::DynamicImage) -> Sprite {
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

pub fn char_to_img(face: &freetype::Face, c: char) -> LuxResult<(image::DynamicImage, CharOffset)> {
    fn buf_to_vec(bf: &[u8], width: u32, height: u32) -> image::DynamicImage {
        let mut v = vec![];
        for y in (0 .. height) {
            for x in (0 .. width) {
                let va = bf[(y * width + x) as usize];
                //print!("{}", va);
                v.push_all(&[va, va, va, va]);
            }
        }
        image::DynamicImage::ImageRgba8(
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

pub fn merge_all<A: ::std::fmt::Debug, I>(mut images: I) ->
(image::DynamicImage, HashMap<A, (texture_packer::Rect, CharOffset)>)
where I: Iterator<Item=(A, LuxResult<(image::DynamicImage, CharOffset)>)>,
      A: Eq + Hash<Hasher> {
    use texture_packer::{Packer, GrowingPacker};
    use std::mem::replace;

    let mut size = 256u32;
    let mut packer: texture_packer::SkylinePacker<_> = {
        let bf = image::DynamicImage::new_rgba8(size, size);
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
            Err(e) => {
            }
        }
    }

    let buf = packer.into_buf();
    (buf, mapping)
}
