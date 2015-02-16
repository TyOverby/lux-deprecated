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

type Advance = (i64, i64);

pub struct FontCache {
    library: freetype::Library,
    faces: HashMap<String, freetype::Face>,
    rendered: HashMap<(String, u32), Rc<RenderedFont>>,
    current: Rc<RenderedFont>
}

pub struct RenderedFont {
    name: String,
    size: u32,
    font_sheet: NonUniformSpriteSheet<char>,
    advances: HashMap<char, Advance>
}

impl FontCache {
    pub fn new<S>(loader: &mut S) -> LuxResult<FontCache> where S: SpriteLoader {
        // Load the default font.
        let bytes = include_bytes!("../resources/SourceCodePro-Regular.ttf");
        let lib = try!(freetype::Library::init());
        let mut face = try!(lib.new_memory_face(bytes, 0));
        let name = "SourceCodePro".to_string();
        let size = 32;

        let rendered = RenderedFont::new(loader, &mut face, name.clone(), size);
        let rendered = Rc::new(try!(rendered));

        let mut fc = FontCache {
            library: lib,
            faces: HashMap::new(),
            rendered: HashMap::new(),
            current: rendered.clone()
        };

        fc.faces.insert(name.clone(), face);
        fc.rendered.insert((name, size), rendered);

        Ok(fc)
    }

    pub fn load(&mut self, name: &str, path: &Path) -> LuxResult<()> {
        if self.faces.contains_key(name) {
            return Ok(());
        }

        let bytes = try!(File::open(path).read_to_end());
        let face = try!(self.library.new_memory_face(&bytes[], 0));
        try!(face.set_pixel_sizes(0, 48));
        self.faces.insert(name.to_string(), face);

        Ok(())
    }

    pub fn use_font<S>(&mut self, loader: &mut S, name: &str, size: u32) -> LuxResult<()>
    where S: SpriteLoader {
        /*
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
*/
        Ok(())
    }

    pub fn draw_onto<S>(&mut self, canvas: &mut S, text: &str) -> LuxResult<()>
    where S: LuxCanvas {
        let face = self.faces.get_mut(&self.current.name).unwrap();
        self.current.render_string(canvas, text, face)
    }
}

impl RenderedFont {
    fn new<S>(loader: &mut S, face: &mut freetype::Face, name: String, size: u32)
    -> LuxResult<RenderedFont> where S: SpriteLoader {
        let (sheet, advs) = try!(gen_sheet(loader, face, size));
        Ok(RenderedFont {
            name: name,
            size: size,
            font_sheet: sheet,
            advances: advs
        })
    }

    fn render_string<C>(&self, canvas: &mut C, text: &str, face: &mut freetype::Face)
    -> LuxResult<()> where C: LuxCanvas {
        let sheet = &self.font_sheet;

        face.set_pixel_sizes(0, self.size);

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
            let adv = self.advances.get(&current).cloned().unwrap_or((0, 0));
            x += adv.0 as f32;
            canvas.sprite(&sheet.get(&current), x, 0.0).draw();

            prev = Some(current);
        }
        Ok(())
    }
}

pub fn gen_sheet<S>(loader: &mut S, face: &freetype::Face, size: u32)
-> LuxResult<(FontSheet, HashMap<char, Advance>)> where S: SpriteLoader {
    let mut v = vec![];
    for i in 1u8 .. 255 {
        v.push(i as char);
    }

    let (texture, map) = merge_all(v.into_iter().map(|c| (c, char_to_img(face, c))));
    let sprite = loader.sprite_from_image(texture);
    let mut sprite = sprite.as_nonuniform_sprite_sheet();
    let mut advances = HashMap::new();
    for (k, (r, offset)) in map {
        sprite.associate(k, (r.x, r.y), (r.w, r.h));
        advances.insert(k, offset);
    }
    Ok((sprite, advances))
}

pub fn char_to_img(face: &freetype::Face, c: char) -> LuxResult<(image::DynamicImage, Advance)> {
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
    let offset = glyph.advance();
    Ok((buf_to_vec(bit.buffer(), bit.width() as u32, bit.rows() as u32), (offset.x, offset.y)))
}

pub fn merge_all<A: ::std::fmt::Debug, I>(mut images: I) ->
(image::DynamicImage, HashMap<A, (texture_packer::Rect, Advance)>)
where I: Iterator<Item=(A, LuxResult<(image::DynamicImage, Advance)>)>,
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
            Err(_) => { }
        }
    }

    let buf = packer.into_buf();
    (buf, mapping)
}
