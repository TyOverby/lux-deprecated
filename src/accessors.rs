use std::cell::RefMut;
use ::font::FontCache;
use super::primitive_canvas::{CachedColorDraw, CachedTexDraw};
use glium;
use reuse_cache;

pub trait HasDisplay {
    fn borrow_display(&self) -> &glium::Display;
    fn clone_display(&self) -> glium::Display {
        self.borrow_display().clone()
    }
}

impl HasDisplay for glium::Display {
    fn borrow_display(&self) -> &glium::Display {
        self
    }
}

pub trait HasPrograms {
    fn texture_shader(&self) -> &glium::Program;
    fn color_shader(&self) -> &glium::Program;
}

pub trait HasFontCache {
    fn font_cache(&self) -> RefMut<FontCache>;
}

pub trait HasSurface {
    type Out: glium::Surface;

    fn surface(&mut self) -> &mut Self::Out;
    fn surface_and_texture_shader(&mut self) -> (&mut Self::Out, &glium::Program);
    fn surface_and_color_shader(&mut self) -> (&mut Self::Out, &glium::Program);
}

pub trait HasDrawCache {
    fn color_draw_cache(&self) -> &Option<CachedColorDraw>;
    fn tex_draw_cache(&self) -> &Option<CachedTexDraw>;

    fn color_draw_cache_mut(&mut self) -> &mut Option<CachedColorDraw>;
    fn tex_draw_cache_mut(&mut self) -> &mut Option<CachedTexDraw>;
}

pub trait Fetch<T> {
    fn fetch(&self) -> reuse_cache::Item<T>;
}
