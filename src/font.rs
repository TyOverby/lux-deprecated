#![allow(unused)]

use std::collections::hash_map::{HashMap, Hasher};
use std::hash::Hash;
use std::rc::Rc;
use std::old_path::Path;
use std::old_io::{File, IoResult};

use image;
use freetype;
use texture_packer;

use super::{LuxCanvas, Sprite, SpriteLoader, TexVertex, NonUniformSpriteSheet, LuxResult};

pub type FontSheet = NonUniformSpriteSheet<char>;

pub struct FontCache {
    library: freetype::Library,
    fonts: HashMap<(String, u32), Rc<FontSheet>>,
    current_font: Rc<FontSheet>,
    current_face: Rc<freetype::Face>,
    name_to_face: HashMap<String, Rc<freetype::Face>>,
}


impl FontCache {
    pub fn new<S>(loader: &mut S) -> LuxResult<FontCache> where S: SpriteLoader {
        let mut fc = FontCache {
            library: try!(freetype::Library::init()),
            fonts: HashMap::new(),

            // This is safe because current_font is set in the call to use_font.
            current_font: unsafe{::std::mem::uninitialized()},
            current_face: unsafe{::std::mem::uninitialized()},
            name_to_face: HashMap::new(),
        };

        let bytes = include_bytes!("../resources/SourceCodePro-Regular.ttf");

        let face = try!(fc.library.new_memory_face(bytes, 0));
        fc.name_to_face.insert("SourceCodePro".to_string(), Rc::new(face));
        fc.use_font(loader, "SourceCodePro", 32);

        Ok(fc)
    }

    pub fn load(&mut self, name: &str, path: &Path) -> LuxResult<()> {
        if self.name_to_face.contains_key(name) {
            return Ok(());
        }

        let bytes = try!(File::open(path).read_to_end());
        let face = try!(self.library.new_memory_face(&bytes[], 0));
        try!(face.set_pixel_sizes(0, 48));
        self.name_to_face.insert(name.to_string(), Rc::new(face));
        Ok(())
    }

    pub fn use_font<S>(&mut self, loader: &mut S, name: &str, size: u32) -> LuxResult<()>
    where S: SpriteLoader {
        let key = (name.to_string(), size);
        if let Some(font_sheet) = self.fonts.get(&key) {
            self.current_font = font_sheet.clone();
            self.current_face = self.name_to_face[name.to_string()].clone();

            return Ok(())
        }

        let error = format!("The font '{}' has not been loaded", name);
        let face = self.name_to_face.get(name).expect(&error[]).clone();
        try!(face.set_pixel_sizes(0, size));
        let sheet = try!(gen_sheet(loader, &*face, size));

        self.fonts.insert(key, sheet.clone());
        self.current_font = sheet;
        self.current_face = face;

        Ok(())
    }

    // TODO: rename this
    pub fn current_face(&self) -> Rc<FontSheet> {
        self.current_font.clone()
    }

    pub fn draw_onto<S>(&self, canvas: &mut S, text: &str) -> LuxResult<()>
    where S: LuxCanvas {
        let face = &self.current_face;
        let sheet = &self.current_font;

        let mut prev: Option<char> = None;
        let mut x = 0.0;
        for current in text.chars() {
            if let Some(prev) = prev {
                let delta = face.get_kerning(face.get_char_index(prev as usize),
                                             face.get_char_index(current as usize),
                                             freetype::face::KerningMode::KerningDefault);
                let delta = try!(delta);
                x += (delta.x >> 6) as f32;
            }
            canvas.sprite(&sheet.get(&current), x, 0.0).draw();

            prev = Some(current);
        }
        Ok(())
    }
}

pub fn gen_sheet<S>(loader: &mut S, face: &freetype::Face, size: u32)
-> LuxResult<Rc<FontSheet>> where S: SpriteLoader {
    let mut v = vec![];
    for i in 1u8 .. 255 {
        v.push(i as char);
    }

    let (texture, map) = merge_all(v.into_iter().map(|c| (c, char_to_img(face, c))));
    let sprite = loader.sprite_from_image(texture);
    let mut sprite = sprite.as_nonuniform_sprite_sheet();
    for (k, (rect, offset)) in map {
        sprite.associate(k, (v.x, v.y), (v.w, v.h));
    }
    Ok(Rc::new(sprite))
}

pub fn char_to_img(face: &freetype::Face, c: char) -> LuxResult<(image::DynamicImage, (isize, isize))> {
    fn buf_to_vec(bf: &[u8], width: u32, height: u32) -> image::DynamicImage {
        let mut v = vec![];
        for y in (0 .. height) {
            for x in (0 .. width) {
                let va = bf[(y * width + x) as usize];
                v.push_all(&[va, va, va, va]);
            }
        }
        image::DynamicImage::ImageRgba8(
            image::ImageBuffer::from_vec(width, height, v).unwrap())
    }

    try!(face.load_char(c as usize, freetype::face::RENDER));
    let glyph = face.glyph();
    let bit = glyph.bitmap();
    let offset = (glyph.advance_x(), glyph.advance_y());
    Ok((buf_to_vec(glyph.buffer(), glyph.width() as u32, glyph.rows() as u32), offset))
}

pub fn merge_all<A: ::std::fmt::Debug, I>(mut images: I) ->
(image::DynamicImage, HashMap<A, (texture_packer::Rect, (isize, isize))>)
where I: Iterator<Item=(A, LuxResult<(image::DynamicImage, (isize, isize))>)>,
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

    //while let Some((a, Ok(img))) = images.next() {
    for (a, (img, offset)) in images {
        match img {
            Ok(img) => {
                let rect = packer.pack_resize(&img, |(x, y)| (x * 2, y * 2));
                mapping.insert(a, rect);
            }
            Err(_) => { }
        }
    }

    let buf = packer.into_buf();
    (buf, mapping)
}
