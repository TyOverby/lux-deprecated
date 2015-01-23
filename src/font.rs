use std::collections::hash_map::HashMap;
use std::rc::Rc;
use std::path::Path;
use std::io::{File, IoResult};

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
