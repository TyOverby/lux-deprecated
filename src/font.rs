use std::collections::hash_map::HashMap;
use std::rc::Rc;
use std::path::Path;
use std::old_io::{File, IoResult};

use image;
use freetype;
use texture_packer;

use super::{Sprite, TexVertex, NonUniformSpriteSheet};

type FontSheet = NonUniformSpriteSheet<char>;

struct FontCache {
    fonts: HashMap<(String, u32), Rc<FontSheet>>,
    current_font: Rc<FontSheet>,
    name_to_contents: HashMap<String, Vec<u8>>
}


impl FontCache {
    fn new() -> FontCache {
        let mut fc = FontCache {
            fonts: HashMap::new(),

            // This is safe because current_font is set in the call to use_font.
            current_font: unsafe{::std::mem::uninitialized()},
            name_to_contents: HashMap::new(),
        };

        let mut font_bytes = Vec::new();
        font_bytes.push_all(include_bytes!("../resources/SourceCodePro-Regular.ttf"));

        fc.name_to_contents.insert("SourceCodePro".to_string(), font_bytes);
        fc.use_font("SourceCodePro", 16);

        fc
    }

    fn load(&mut self, name: &str, path: &Path) -> IoResult<()> {
        self.name_to_contents.insert(
            name.to_string(), try!(File::open(path).read_to_end()));
        Ok(())
    }

    fn use_font(&mut self, name: &str, size: u32) {
        let key = (name.to_string(), size);

        let todo = match self.fonts.get(&key) {
            Some(f) => {
                self.current_font = f.clone();
                None
            }
            None => {
                let error = format!("The font '{}' has not been loaded", name);
                let contents = self.name_to_contents.get(name).expect(&error[]);
                let ff = FontCache::load_ttf(&contents[], size);
                Some(ff)
            }
        };

        if let Some(ff) = todo {
            self.fonts.insert(key, ff);
        }
    }

    fn load_ttf(contents: &[u8], size: u32) -> Rc<FontSheet> {
        unimplemented!()
    }
}

pub fn char_to_img(face: &mut freetype::Face, c: char) -> image::DynamicImage {
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

    face.load_char(c as usize, freetype::face::RENDER).unwrap();
    let g = face.glyph().bitmap();
    buf_to_vec(g.buffer(), g.width() as u32, g.rows() as u32)
}

pub fn merge_all<I: Iterator<Item=image::DynamicImage>>(mut images: I) -> image::DynamicImage {
    use texture_packer::{Packer, GrowingPacker};
    use std::mem::replace;

    let mut size = 256u32;
    let mut packer: texture_packer::SkylinePacker<_> = {
        let bf = image::DynamicImage::new_rgba8(size, size);
        Packer::new(bf)
    };
    packer.set_margin(5);

    for img in images {
        packer.pack_resize(&img, |(x, y)| (x * 2, y * 2));
    }

    packer.into_buf()
}

