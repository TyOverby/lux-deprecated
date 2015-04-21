use std::cell::RefMut;
use ::font::FontCache;
use glium;
use super::primitive_canvas::{CachedColorDraw, CachedTexDraw};

pub trait HasDisplay {
    fn borrow_display(&self) -> &glium::Display;
    fn clone_display(&self) -> glium::Display {
        self.borrow_display().clone()
    }
}

pub trait HasPrograms {
    fn texture_shader(&self) -> &glium::Program;
    fn color_shader(&self) -> &glium::Program;
}

pub trait HasFontCache {
    fn font_cache(&self) -> RefMut<Option<FontCache>>;
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
