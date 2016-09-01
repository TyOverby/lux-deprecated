//! Accessors are a low-level detail that Lux uses to
//! enable code-reuse between components that share behaviors or
//! capabilities.

use std::rc::Rc;
use std::cell::RefCell;

use super::primitive_canvas::{
    CachedColorDraw,
    CachedTexDraw,
    DrawParamModifier,
    StencilState
};
use super::sprite::Sprite;
use glium;
use poison_pool;
use font_atlas::cache::FontCache;

pub trait StateLike {
    fn state_fields(&mut self) -> StateFields;
}

pub trait DrawLike {
    type Surface: glium::Surface;
    fn draw_fields(&mut self) -> DrawFields<Self::Surface>;
}

pub struct DrawFields<'a, S: glium::Surface + 'a> {
    pub display: &'a mut glium::Display,
    pub draw_param_mod: &'a mut DrawParamModifier,
    pub scissor: &'a mut Option<(u32, u32, u32, u32)>,
    pub stencil_state: &'a mut StencilState,
    pub font_cache: &'a Rc<RefCell<FontCache<Sprite>>>,
    pub texture_shader: &'a Rc<glium::Program>,
    pub color_shader: &'a Rc<glium::Program>,
    pub color_draw_cache: &'a mut Option<CachedColorDraw>,
    pub tex_draw_cache: &'a mut Option<CachedTexDraw>,
    pub surface: &'a mut S,
}

pub struct StateFields<'a> {
    pub display: &'a mut glium::Display,
    pub font_cache: &'a Rc<RefCell<FontCache<Sprite>>>,
    pub texture_shader: &'a Rc<glium::Program>,
    pub color_shader: &'a Rc<glium::Program>,
}

/// Implemented on objects that can hand off items from a cache.
pub trait Fetch<T> {
    /// Fetches an item.
    fn fetch(&self) -> poison_pool::Item<T>;
}
