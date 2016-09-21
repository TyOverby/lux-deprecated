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
    fn state_fields(&self) -> StateFields;
}

pub trait DrawLike: StateLike {
    type Surface: glium::Surface;
    fn draw_fields(&mut self) -> DrawFields<Self::Surface>;
    fn draw_fields_ref(&self) -> DrawFieldsRef<Self::Surface>;
}

pub struct StateFields<'a> {
    pub display: &'a glium::Display,
    pub font_cache: &'a Rc<RefCell<FontCache<Sprite>>>,
    pub texture_shader: &'a Rc<glium::Program>,
    pub color_shader: &'a Rc<glium::Program>,
}

pub struct DrawFields<'a, S: glium::Surface + 'a> {
    pub display: &'a mut glium::Display,
    pub scissor: &'a mut Option<(u32, u32, u32, u32)>,
    pub stencil_state: &'a mut StencilState,
    pub font_cache: &'a Rc<RefCell<FontCache<Sprite>>>,
    pub texture_shader: &'a Rc<glium::Program>,
    pub color_shader: &'a Rc<glium::Program>,
    pub color_draw_cache: &'a mut Option<CachedColorDraw>,
    pub tex_draw_cache: &'a mut Option<CachedTexDraw>,
    pub surface: &'a mut S,
    pub matrix: &'a mut [[f32; 4]; 4],
}

pub struct DrawFieldsRef<'a, S: glium::Surface + 'a> {
    pub display: &'a glium::Display,
    pub scissor: &'a Option<(u32, u32, u32, u32)>,
    pub stencil_state: &'a StencilState,
    pub font_cache: &'a Rc<RefCell<FontCache<Sprite>>>,
    pub texture_shader: &'a Rc<glium::Program>,
    pub color_shader: &'a Rc<glium::Program>,
    pub color_draw_cache: &'a Option<CachedColorDraw>,
    pub tex_draw_cache: &'a Option<CachedTexDraw>,
    pub surface: &'a S,
    pub matrix: &'a [[f32; 4]; 4],
}

impl <'a, S: glium::Surface + 'a> DrawFields<'a, S> {
    pub fn take_scissor(&mut self) -> Option<(u32, u32, u32, u32)> {
        self.scissor.take()
    }
    pub fn set_scissor(&mut self, scissor: Option<(u32, u32, u32, u32)>) {
        *self.scissor = scissor;
    }
    pub fn set_stencil_state(self, stencil_state: StencilState) {
        *self.stencil_state = stencil_state;
    }
}

/// Implemented on objects that can hand off items from a cache.
pub trait Fetch<T> {
    /// Fetches an item.
    fn fetch(&self) -> poison_pool::Item<T>;
}
