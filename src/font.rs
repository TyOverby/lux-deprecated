use std::collections::hash_map::HashMap;
use std::rc::Rc;
use std::path::Path;
use std::io::{File, IoResult};

use super::{Sprite, TexVertex};

struct FontCache {
    fonts: HashMap<(String, u32), Rc<FontFace>>,
    current_font: Rc<FontFace>,
    name_to_contents: HashMap<String, Vec<u8>>
}

struct FontFace {
    size: u32,
    texture: Sprite,
    char_info: HashMap<char, CharacterInfo>
}

struct CharacterInfo {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    sprite: Sprite
}


impl FontCache {
    fn new() -> FontCache {
        let mut fc = FontCache {
            fonts: HashMap::new(),
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
        self.name_to_contents.insert(name.to_string(),
                                     try!(File::open(path).read_to_end()));
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
                let ff = FontFace::new(&contents[], size);
                Some(ff)
            }
        };

        if let Some(ff) = todo {
            self.fonts.insert(key, ff);
        }
    }
}

impl FontFace {
    fn new(contents: &[u8], size: u32) -> Rc<FontFace> {
        unimplemented!()
    }

    fn generate_buf(&self, message: &str) -> (Vec<TexVertex>, Vec<u32>, Sprite) {
        let mut vbuffer = Vec::with_capacity(message.len() * 4);
        let mut ibuffer = Vec::with_capacity(message.len());
        let mut idx_count = 0;

        let mut x = 0.0;
        for c in message.chars() {
            let info =
                self.char_info.get(&c)
                              .or_else(||self.char_info.get(&'?')).unwrap();
            let (vtxs, idxs) = info.sprite.zeroed_vertices();

            for mut vtx in vtxs.into_iter() {
                vtx.pos[0] = vtx.pos[0] * info.width as f32 + x;
                vtx.pos[1] = vtx.pos[0] * info.height as f32;
                vbuffer.push(vtx);
            }

            for idx in idxs.into_iter() {
                ibuffer.push(idx + idx_count);
            }


            x += info.width as f32;
            idx_count = vbuffer.len() as u32;
        }
        (vbuffer, ibuffer, self.texture.clone())
    }
}
