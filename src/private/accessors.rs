//! Accessors are a low-level detail that Lux uses to
//! enable code-reuse between components that share behaviors or
//! capabilities.

use super::primitive_canvas::{
    CachedColorDraw,
    CachedTexDraw,
    DrawParamModifier,
    StencilState
};
use glium;
use poison_pool;

/// Implemented on objects that have a glutin Display.
pub trait HasDisplay {
    /// Borrows the display.
    fn borrow_display(&self) -> &glium::Display;
    /// Takes a clone of the display.
    fn clone_display(&self) -> glium::Display {
        self.borrow_display().clone()
    }
}

impl HasDisplay for glium::Display {
    fn borrow_display(&self) -> &glium::Display {
        self
    }
}

/// Implemented on objects that have a scissor stencil.
pub trait DrawParamMod {
    /// Returns a reference to the current draw parameter modifier
    fn draw_param_mod(&self) -> &DrawParamModifier;
    /// Returns a mutable reference to the current draw parameter modifier
    fn draw_param_mod_mut(&mut self) -> &mut DrawParamModifier;

    /// Return a reference to the scissor.
    fn scissor(&self) -> Option<(u32, u32, u32, u32)> {
        self.draw_param_mod().scissor.clone()
    }

    /// Takes and returns the scissor.
    fn take_scissor(&mut self) -> Option<(u32, u32, u32, u32)> {
        self.draw_param_mod_mut().scissor.take()
    }

    /// Sets the scissor.
    fn set_scissor(&mut self, s: Option<(u32, u32, u32, u32)>) {
        *(&mut self.draw_param_mod_mut().scissor) = s
    }

    ///
    fn stencil_state(&self) -> StencilState {
        self.draw_param_mod().stencil_state
    }

    ///
    fn set_stencil_state(&mut self, stencil_state: StencilState) {
        (*self.draw_param_mod_mut()).stencil_state = stencil_state;
    }
}

/// Implemented on objects that have a reference to the different
/// shader programs.
pub trait HasPrograms {
    /// Returns a reference to the texture shader.
    fn texture_shader(&self) -> &glium::Program;
    /// Returns a reference to the color shader.
    fn color_shader(&self) -> &glium::Program;
}

/// Implemented on objects that contain a gluium `Surface`.
pub trait HasSurface {
    /// The surface itself.
    type Out: glium::Surface;

    /// Returns a mutable reference to the surface.
    fn surface(&mut self) -> &mut Self::Out;
    /// Returns a mutable reference to the surface and the texture shader.
    fn surface_and_texture_shader(&mut self) -> (&mut Self::Out, &glium::Program);
    /// Returns a mutable reference to the surface and the color shader.
    fn surface_and_color_shader(&mut self) -> (&mut Self::Out, &glium::Program);
}

/// Implemented on objects that have a draw cache.
pub trait HasDrawCache {
    /// Returns a reference to the current color draw cache.
    fn color_draw_cache(&self) -> &Option<CachedColorDraw>;
    /// Returns a reference to the current texture draw cache.
    fn tex_draw_cache(&self) -> &Option<CachedTexDraw>;

    /// Returns a mutable reference to the current color draw cache.
    fn color_draw_cache_mut(&mut self) -> &mut Option<CachedColorDraw>;
    /// Returns a mutable reference to the current texture draw cache.
    fn tex_draw_cache_mut(&mut self) -> &mut Option<CachedTexDraw>;
}

/// Implemented on objects that can hand off items from a cache.
pub trait Fetch<T> {
    /// Fetches an item.
    fn fetch(&self) -> poison_pool::Item<T>;
}
