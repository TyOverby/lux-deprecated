use std::collections::hash_map::HashMap;
use std::rc::Rc;
use std::path::Path;
use std::io::{File, IoResult};

use super::Sprite;

struct FontCache {
    fonts: HashMap<(String, u32), Rc<FontFace>>,
    current_font: Rc<FontFace>,
    name_to_path: HashMap<String, Path>,
    path_to_contents: HashMap<Path, Vec<u8>>
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
    fn new(default_name: &str, default_size: u32, default_path: Path) -> FontCache {
        let ff = Rc::new(FontFace::new(&default_path, default_size));
        let mut map = HashMap::new();
        map.insert((default_name.to_string(), default_size), ff.clone());
        FontCache {
            fonts: map,
            current_font: ff
        }
    }

    fn load(&mut self, name: &str, path: &Path) -> IoResult<()> {
        self.name_to_path.insert(name.to_string(), path.clone());
        self.path_to_contents.insert(path.clone(), try!(File::open(path).read_to_end()));
    }

    fn use_font(&mut self, name: &str, size: i32) {

    }
}

impl FontFace {
    fn new(path: &Path, size: u32) -> FontFace {
        unimplemented!()
    }
}
