use std::cell::RefMut;

use glium;
use ::font::FontCache;

pub trait HasDisplay {
    fn borrow_display(&self) -> &glium::Display;
    fn clone_display(&self) -> glium::Display {
        self.borrow_display().clone()
    }
}

pub trait HasFontCache {
    fn font_cache(&self) -> RefMut<Option<FontCache>>;
}

