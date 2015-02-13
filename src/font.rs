#![allow(unused)]

use std::collections::hash_map::{HashMap, Hasher};
use std::hash::Hash;
use std::rc::Rc;
use std::old_path::Path;
use std::old_io::{File, IoResult};

use image;
use freetype;
use texture_packer;

use super::{Sprite, SpriteLoader, TexVertex, NonUniformSpriteSheet, LuxResult};

type FontSheet = NonUniformSpriteSheet<char>;

struct FontCache {
    library: freetype::Library,
    fonts: HashMap<(String, u32), Rc<FontSheet>>,
    current_font: Rc<FontSheet>,
    name_to_face: HashMap<String, freetype::Face>,
}


impl FontCache {
    fn new<S>(loader: &mut S) -> LuxResult<FontCache> where S: SpriteLoader {
        let mut fc = FontCache {
            library: try!(freetype::Library::init()),
            fonts: HashMap::new(),

            // This is safe because current_font is set in the call to use_font.
            current_font: unsafe{::std::mem::uninitialized()},
            name_to_face: HashMap::new(),
        };

        let bytes = include_bytes!("../resources/SourceCodePro-Regular.ttf");

        let face = try!(fc.library.new_memory_face(bytes, 0));
        fc.name_to_face.insert("SourceCodePro".to_string(), face);
        fc.use_font(loader, "SourceCodePro", 16);

        Ok(fc)
    }

    fn load(&mut self, name: &str, path: &Path) -> LuxResult<()> {
        let bytes = try!(File::open(path).read_to_end());
        let face = try!(self.library.new_memory_face(&bytes[], 0));
        self.name_to_face.insert(name.to_string(), face);
        Ok(())
    }

    fn use_font<S>(&mut self, loader: &mut S, name: &str, size: u32) -> LuxResult<()>
    where S: SpriteLoader {
        let key = (name.to_string(), size);

        let todo = match self.fonts.get(&key) {
            Some(f) => {
                self.current_font = f.clone();
                None
            }
            None => {
                let error = format!("The font '{}' has not been loaded", name);
                let face = self.name_to_face.get_mut(name).expect(&error[]);
                let sheet = try!(gen_sheet(loader, face, size));
                Some(sheet)
            }
        };

        if let Some(ff) = todo {
            self.fonts.insert(key, ff);
        }
        Ok(())
    }

}

fn gen_sheet<S>(loader: &mut S, face: &mut freetype::Face, size: u32)
-> LuxResult<Rc<FontSheet>> where S: SpriteLoader {
    let mut v = vec![];
    for i in 1u8 .. 255 {
        v.push(i as char);
    }

    let (texture, map) = merge_all(v.into_iter().map(|c| (c, char_to_img(face, c))));
    let sprite = loader.sprite_from_image(texture);
    let mut sprite = sprite.as_nonuniform_sprite_sheet();
    for (k, v) in map {
        sprite.associate(k, (v.x, v.y), (v.w, v.h));
    }
    Ok(Rc::new(sprite))
}

pub fn char_to_img(face: &mut freetype::Face, c: char) -> LuxResult<image::DynamicImage> {
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
    let g = face.glyph().bitmap();
    Ok(buf_to_vec(g.buffer(), g.width() as u32, g.rows() as u32))
}

pub fn merge_all<A, I>(mut images: I) ->
(image::DynamicImage, HashMap<A, texture_packer::Rect>)
where I: Iterator<Item=(A, LuxResult<image::DynamicImage>)>,
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

    while let Some((a, Ok(img))) = images.next() {
        let rect = packer.pack_resize(&img, |(x, y)| (x * 2, y * 2));
        mapping.insert(a, rect);
    }

    (packer.into_buf(), mapping)
}

/*
pub fn load_face(lib: &freetype::Library,contents: &[u8], size: u32) ->
LuxResult<image::DynamicImage> {
    let mut face = try!(lib.new_memory_face(contents, 0));
    face.set_pixel_sizes(0, size);

    let ascii = (0u8 .. 255).map(|c| char_to_img(&mut face, c as char));

    merge_all(v.into_iter().map(|c| char_to_img(&mut face, c)));
}
*/


// Iterator<Result<T, E>> -> Result<Iterator<T>, E>
