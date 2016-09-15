use super::primitive_canvas::{PrimitiveCanvas, StencilState, StencilType};
use super::types::Float;
use super::gfx_integration::{ColorVertex, TexVertex};
use super::color::{Color, rgb};
use super::raw::Transform;
use super::sprite::Sprite;
use super::accessors::DrawLike;
use ::LuxResult;

use ::vecmath;

use glium::index::PrimitiveType::{TriangleFan, TrianglesList, Points};

pub trait Drawable {
    fn draw<C: Canvas>(self, target: &mut C) -> LuxResult<()>;
}

/// Canvas is the main trait for drawing in Lux.  It supports all operations
/// that paint to the screen or to a buffer.
pub trait Canvas: DrawLike + Sized {
    fn draw<O: Drawable>(&mut self, subject: O) -> LuxResult<()> {
        subject.draw(self)
    }

    /// Returns the size of the canvas as a pair of (width, height).
    fn size(&self) -> (Float, Float);

    /// Returns the size of the canvas in integer form.
    fn size_i(&self) -> (i32, i32) {
        let (w, h) = self.size();
        (w as i32, h as i32)
    }

    /// Returns the width of the canvas.
    fn width(&self) -> Float {
        match self.size() {
            (w, _) => w
        }
    }

    /// Returns the height of the canvas.
    fn height(&self) -> Float {
        match self.size() {
            (_, h) => h
        }
    }

    /// Returns the width of the canvas in integer form.
    fn width_i(&self) -> i32 {
        self.width() as i32
    }

    /// Returns the height of the canvas in integer form.
    fn height_i(&self) -> i32 {
        self.width() as i32
    }

    /// Clears the canvas with a solid color.
    ///
    /// ```rust,no_run
    ///# extern crate lux;
    /// use lux::prelude::*;
    ///# fn main() {
    ///
    /// let mut window = Window::new().unwrap();
    /// let mut frame = window.frame();
    /// // Clear the screen with purple.
    /// frame.clear(rgb(1.0, 0.0, 1.0));
    ///# }
    /// ```
    fn clear<C: Color>(&mut self, color: C) {
        PrimitiveCanvas::clear(self, color);
    }

    /// Evaluates the function with a canvas that will only draw into the
    /// provided rectangle.
    fn with_scissor<F, R>(&mut self, x: u32, y: u32, w: u32, h: u32, f: F) -> R
    where F: FnOnce(&mut Self) -> R {
        // Flush things that we don't want scissored.
        self.flush_draw().unwrap();

        let view_height = self.height() as u32;
        let old = self.draw_fields().take_scissor();
        // TODO: merge these rectangles
        self.draw_fields().set_scissor(Some((x, view_height - h - y, w, h)));
        let res = f(self);
        self.flush_draw().unwrap();
        self.draw_fields().set_scissor(old);
        res
    }

    /// Executes a drawing function where all drawing is done on the
    /// stencil buffer.
    fn draw_to_stencil<R, S>(&mut self, typ: StencilType, stencil_fn: S) -> R
    where S: FnOnce(&mut Self) -> R {
        self.flush_draw().unwrap();
        self.draw_fields().set_stencil_state(StencilState::DrawingStencil(typ));

        let res1 = stencil_fn(self);
        self.flush_draw().unwrap();
        self.draw_fields().set_stencil_state(StencilState::DrawingWithStencil);
        res1
    }

    /// Clears the stencil buffer allowing all draws to go though.
    ///
    /// When called with `StencilType::Allow`, the stencil buffer will
    /// be cleared allowing all future draws to pass through until
    /// `draw_to_stencil` is called with `StencilType::Deny`.
    ///
    /// Whene called with `StencilType::Deny`, the stencil buffer will be
    /// filled, preventing all future draws to fail until `draw_to_stencil`
    /// is called with `StencilType::Allow`.
    fn clear_stencil(&mut self, typ: StencilType) {
        self.flush_draw().unwrap();
        match typ {
            StencilType::Allow => {
                PrimitiveCanvas::clear_stencil(self, 1);
                self.draw_fields().set_stencil_state(StencilState::None);
            }
            StencilType::Deny => {
                PrimitiveCanvas::clear_stencil(self, 0);
                self.draw_fields().set_stencil_state(StencilState::DrawingWithStencil);
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Rectangle {
    pub x: Float,
    pub y: Float,
    pub w: Float,
    pub h: Float,
    pub color: [f32; 4],
    pub transform: Option<[[Float; 4]; 4]>
}

impl Default for Rectangle {
    fn default() -> Rectangle {
        Rectangle {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            color: rgb(0.0, 0.0, 0.0),
            transform: None
        }
    }
}

impl Drawable for Rectangle {
    fn draw<C: Canvas>(self, canvas: &mut C) -> LuxResult<()> {
        let vertices = [
            ColorVertex{ pos: [self.x + self.w, self.y], color: self.color },
            ColorVertex{ pos: [self.x, self.y], color: self.color },
            ColorVertex{ pos: [self.x, self.y + self.h], color: self.color },
            ColorVertex{ pos: [self.x + self.w, self.y + self.h], color: self.color },
        ];

        let idxs = [0, 1, 2, 0, 2, 3];

        canvas.draw_colored(TrianglesList, &vertices[..], Some(&idxs[..]), self.transform)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Square {
    pub x: Float,
    pub y: Float,
    pub size: Float,
    pub color: [f32; 4],
    pub transform: Option<[[Float; 4]; 4]>
}

impl Default for Square {
    fn default() -> Square {
        Square {
            x: 0.0,
            y: 0.0,
            size: 0.0,
            color: rgb(0.0, 0.0, 0.0),
            transform: None
        }
    }
}

impl Drawable for Square{
    fn draw<C: Canvas>(self, canvas: &mut C) -> LuxResult<()> {
        Rectangle {x: self.x, y: self.y, w: self.size, h: self.size, color: self.color, transform: self.transform}.draw(canvas)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Ellipse {
    pub x: Float,
    pub y: Float,
    pub w: Float,
    pub h: Float,
    pub color: [f32; 4],
    pub segments: Option<u32>,
    pub transform: Option<[[Float; 4]; 4]>
}


impl Default for Ellipse {
    fn default() -> Ellipse {
        Ellipse {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            color: rgb(0.0, 0.0, 0.0),
            segments: None,
            transform: None,
        }
    }
}

impl Drawable for Ellipse {
    fn draw<C: Canvas>(self, canvas: &mut C) -> LuxResult<()> {
        const OPT_LINE_LENGTH: u16 = 15;

        use std::f32::consts::PI;

        let largest_radius = self.w.max(self.h);
        let delta_theta = self.segments.map(|segment_count| (2.0 * PI) / (segment_count as Float))
                                       .unwrap_or(OPT_LINE_LENGTH as f32 / largest_radius);
        let mut vertices = vec![];

        let mut theta = 0.0;
        while theta <= 2.0 * PI {
            let (mut x, mut y) = (theta.sin(), theta.cos());
            x *= self.w / 2.0;
            x += self.w;

            y *= self.h / 2.0;
            y += self.h;

            vertices.push(ColorVertex { pos: [x + self.x, y + self.y], color: self.color });
            theta += delta_theta;
        }

        canvas.draw_colored(TriangleFan, &vertices[..], None, self.transform)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Circle {
    pub x: Float,
    pub y: Float,
    pub size: Float,
    pub color: [f32; 4],
    pub segments: Option<u32>,
    pub transform: Option<[[Float; 4]; 4]>
}


impl Default for Circle {
    fn default() -> Circle {
        Circle {
            x: 0.0,
            y: 0.0,
            size: 0.0,
            color: rgb(0.0, 0.0, 0.0),
            segments: None,
            transform: None,
        }
    }
}

impl Drawable for Circle {
    fn draw<C: Canvas>(self, canvas: &mut C) -> LuxResult<()> {
        Ellipse { x: self.x, y: self.y, w: self.size, h: self.size, color: self.color, segments: self.segments, transform: self.transform }.draw(canvas)
    }
}

#[derive(Copy, Clone)]
pub struct Picture<'a> {
    pub sprite: Option<&'a Sprite>,
    pub x: Float,
    pub y: Float,
    pub size: Option<(Float, Float)>,
    pub color: [f32; 4],
    pub transform: Option<[[Float; 4]; 4]>,
}

impl Default for Picture<'static> {
    fn default() -> Picture<'static> {
        Picture {
            sprite: None,
            x: 0.0,
            y: 0.0,
            size: None,
            color: [1.0, 1.0, 1.0, 1.0],
            transform: None,
        }
    }
}

impl <'a> Drawable for Picture<'a> {
    fn draw<C: Canvas>(self, canvas: &mut C) -> LuxResult<()> {
        let sprite = match self.sprite {
            Some(sprite) => sprite,
            None => return Ok(()),
        };

        let bounds = sprite.bounds();

        let top_left = bounds[0];
        let top_right = bounds[1];
        let bottom_left = bounds[2];
        let bottom_right = bounds[3];

        let (w, h) = match self.size {
            Some((w, h)) => (w, h),
            None => (sprite.width(), sprite.height()),
        };
        let (x, y) = (self.x, self.y);

        let tex_vs = vec![
            TexVertex {pos: [x + w, y], tex_coords: top_right},
            TexVertex {pos: [x, y], tex_coords: top_left},
            TexVertex {pos: [x, y + h], tex_coords: bottom_left},
            TexVertex {pos: [x + w, y + h], tex_coords: bottom_right},
        ];

        let idxs = [0, 1, 2, 0, 2, 3];

        canvas.draw_tex(
            TrianglesList,
            &tex_vs[..],
            Some(&idxs[..]),
            self.transform,
            sprite.texture(),
            Some(self.color))
    }
}

#[derive(Copy, Clone)]
pub struct Line {
    pub start: (Float, Float),
    pub end: (Float, Float),
    pub thickness: Float,
    pub color: [Float; 4],
    pub transform: Option<[[Float; 4]; 4]>
}

impl Default for Line {
    fn default() -> Line {
        Line {
            start: (0.0, 0.0),
            end: (0.0, 0.0),
            thickness: 1.0,
            color: rgb(0.0, 0.0, 0.0),
            transform: None,
        }
    }
}

impl Drawable for Line {
    fn draw<C: Canvas>(self, canvas: &mut C) -> LuxResult<()> {
        let (x1, y1) = self.start;
        let (x2, y2) = self.end;

        let dx = x2 - x1;
        let dy = y2 - y1;
        let dist = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx);

        let mut transformation = vecmath::mat4_id();
        transformation.translate(x1, y1).rotate(angle).translate(0.0, - self.thickness / 2.0);
        if let Some(other_trans) = self.transform {
            transformation.apply_matrix(other_trans);
        }

        Rectangle { x: x1, y: y1, w: dist, h: self.thickness, transform: Some(transformation), color: self.color}.draw(canvas)
    }
}

#[derive(Copy, Clone)]
pub struct Pixels<'a> {
    pub pixels: &'a [ColorVertex],
    pub transform: Option<[[Float; 4]; 4]>
}

impl Default for Pixels<'static> {
    fn default() -> Pixels<'static> {
        Pixels {
            pixels: &[],
            transform: None,
        }
    }
}

impl <'a> Drawable for Pixels<'a> {
    fn draw<C: Canvas>(self, canvas: &mut C) -> LuxResult<()> {
        let mut transf = vecmath::mat4_id();
        transf.translate(0.5, 0.5); // Correctly align
        if let Some(other_trans) = self.transform {
            transf.apply_matrix(other_trans);
        }

        canvas.draw_colored(Points, self.pixels, None, Some(transf))
    }
}
