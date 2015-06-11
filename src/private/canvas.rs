use super::primitive_canvas::{PrimitiveCanvas, StencilState, StencilType};
use super::accessors::DrawParamMod;
use super::types::Float;
use super::gfx_integration::{ColorVertex, TexVertex};
use super::color::Color;
use super::raw::{Colored, Transform};
use super::sprite::Sprite;

use glium::index::PrimitiveType::{TriangleFan, TrianglesList, Points};

use vecmath;

struct BasicFields<'a, C: 'a> {
    fill_color: [Float; 4],
    stroke_color: Option<[Float; 4]>,
    border: Float,
    transform: [[Float; 4]; 4],

    pos: (Float, Float),
    size: (Float, Float),
    canvas: &'a mut C
}

/// An ellipse that can be drawn to the screen.
#[must_use = "Ellipses only contain context, and must be drawn with `fill()`, `stroke()`, or `fill_stroke()`"]
pub struct Ellipse<'a, C: 'a> {
    fields: BasicFields<'a, C>,
    spokes: u16
}

/// A Rectangle that can be drawn to the screen.
#[must_use = "Rectangles only contain context, and must be drawn with `fill()`, `stroke()`, or `fill_stroke()`"]
pub struct Rectangle<'a, C: 'a> {
    fields: BasicFields<'a, C>,
}

/// A sprite that can be drawn to the screen.
#[must_use = "Sprites only contain context, and must be drawn with `draw()`"]
pub struct ContainedSprite<'a, C: 'a>  {
    fields: BasicFields<'a, C>,
    sprite: Sprite
}

/// LuxCanvas is the main trait for drawing in Lux.  It supports all operations
/// that paint to the screen or to a buffer.
pub trait LuxCanvas: PrimitiveCanvas + Colored + Transform + DrawParamMod+ Sized {
    /// Returns the size of the canvas as a pair of (width, height).
    fn size(&self) -> (Float, Float);

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
        let old = self.take_scissor();
        // TODO: merge these rectangles
        self.set_scissor(Some((x, view_height - h - y, w, h)));
        let res = f(self);
        self.flush_draw().unwrap();
        self.set_scissor(old);
        res
    }

    /// Executes a drawing function where all drawing is done on the
    /// stencil buffer.
    fn draw_to_stencil<R, S>(&mut self, typ: StencilType, stencil_fn: S) -> R
    where S: FnOnce(&mut Self) -> R {
        self.flush_draw().unwrap();
        self.set_stencil_state(StencilState::DrawingStencil(typ));

        let res1 = stencil_fn(self);
        self.flush_draw().unwrap();
        self.set_stencil_state(StencilState::DrawingWithStencil);
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
                self.set_stencil_state(StencilState::None);
            }
            StencilType::Deny => {
                PrimitiveCanvas::clear_stencil(self, 0);
                self.set_stencil_state(StencilState::DrawingWithStencil);
            }
        }
    }

    /// Returns a rectangle with the given dimensions and position.
    ///
    /// ```rust,no_run
    ///# extern crate lux;
    /// use lux::prelude::*;
    ///# fn main() {
    ///
    /// let mut window = Window::new().unwrap();
    /// let mut frame = window.frame();
    /// let (x, y, w, h) = (0.0, 5.0, 50.0, 70.0);
    /// frame.rect(x, y, w, h)
    ///      .color(rgb(100, 200, 255))
    ///      .fill();
    ///# }
    /// ```
    fn rect<'a>(&'a mut self, x: Float, y: Float, w: Float, h: Float) -> Rectangle<'a, Self> {
        let c = self.get_color();
        Rectangle::new(self, (x, y), (w, h), c)
    }

    /// Returns a square with the given dimensions and position.
    ///
    /// ```rust,no_run
    /// use lux::prelude::*;
    ///# extern crate lux;
    ///# fn main() {
    ///
    /// let mut window = Window::new().unwrap();
    /// let mut frame = window.frame();
    /// let (x, y, size) = (0.0, 5.0, 50.0);
    /// frame.square(x, y, size)
    ///      .color(rgb(100, 200, 255))
    ///      .fill();
    ///# }
    /// ```
    fn square<'a>(&'a mut self, x: Float, y: Float, size: Float) -> Rectangle<'a, Self> {
        let c = self.get_color();
        Rectangle::new(self, (x, y), (size, size), c)
    }

    /// Returns an ellipse with the given dimensions and position.
    ///
    /// ```rust,no_run
    ///# extern crate lux;
    /// use lux::prelude::*;
    ///# fn main() {
    ///
    /// let mut window = Window::new().unwrap();
    /// let mut frame = window.frame();
    /// let (x, y, w, h) = (0.0, 5.0, 50.0, 70.0);
    /// frame.ellipse(x, y, w, h)
    ///      .color(rgb(100, 200, 255))
    ///      .fill();
    ///# }
    /// ```
    fn ellipse<'a>(&'a mut self, x: Float, y: Float, w: Float, h: Float) -> Ellipse<'a, Self> {
        let c = self.get_color();
        Ellipse::new(self, (x, y), (w, h), c)
    }

    /// Returns an circle with the given dimensions and position.
    ///
    /// ```rust,no_run
    ///# extern crate lux;
    /// use lux::prelude::*;
    ///# fn main() {
    ///
    /// let mut window = Window::new().unwrap();
    /// let mut frame = window.frame();
    /// let (x, y, size) = (0.0, 5.0, 50.0);
    /// frame.circle(x, y, size)
    ///      .color(rgb(100, 200, 255))
    ///      .fill();
    ///# }
    /// ```
    fn circle<'a>(&'a mut self, x: Float, y: Float, size: Float) -> Ellipse<'a, Self> {
        let c = self.get_color();
        Ellipse::new(self, (x, y), (size, size), c)
    }

    /// Draws a 1-pixel colored point to the screen at a position.
    ///
    /// This is *not* the same as setting a "pixel" because the point can
    /// be moved by transformations on the Frame.
    ///
    /// ```rust,no_run
    ///# extern crate lux;
    /// use lux::prelude::*;
    ///# fn main() {
    ///
    /// let mut window = Window::new().unwrap();
    /// let mut frame = window.frame();
    /// let (x, y) = (10.0, 20.0);
    /// frame.draw_point(x, y, lux::color::RED);
    ///# }
    /// ```
    fn draw_point<C: Color>(&mut self, x: Float, y: Float, color: C) {
        let vertex = ColorVertex {
            pos: [x, y],
            color: color.to_rgba(),
        };
        self.draw_colored(Points, &[vertex][..], None, None).unwrap();
    }

    /// Draws a sequence of colored points with the size of 1 pixel.
    ///
    /// ```rust,no_run
    ///# extern crate lux;
    /// use lux::prelude::*;
    /// use lux::graphics::ColorVertex;
    ///# fn main() {
    ///
    /// let mut window = Window::new().unwrap();
    /// let mut frame = window.frame();
    /// let points = [
    ///     ColorVertex {
    ///         pos: [10.0, 15.0],
    ///         color: rgb(255, 0, 0)
    ///     },
    ///     ColorVertex {
    ///         pos: [15.0, 10.0],
    ///         color: rgb(0, 255, 0)
    ///     },
    /// ];
    /// frame.draw_points(&points[..]);
    ///# }
    /// ```
    fn draw_points(&mut self, pixels: &[ColorVertex]) {
        let mut transf = vecmath::mat4_id();
        transf.translate(0.5, 0.5); // Correctly align
        self.draw_colored(Points, &pixels[..], None, Some(transf)).unwrap();
    }

    /// Draws a single line from `start` to `end` with a
    /// thickness of `line_size`.
    fn draw_line(&mut self, _x1: Float, _y1: Float, _x2: Float, _y2: Float, _line_size: Float) {
        unimplemented!();
    }

    /// Draws a series of lines from each point to the next with a thickness
    /// of `line_size`.
    fn draw_lines<I: Iterator<Item = (Float, Float)>>(&mut self, mut _positions: I, _line_size: Float) {
        unimplemented!();
    }

    /// Draws an arc centered at `pos` from `angle1` to `angle_2` with a
    /// thickness of `line_size`.
    fn draw_arc(&mut self, _pos: (Float, Float), _radius: Float, _angle1: Float,
                _angle2: Float, _line_size: Float) {
        unimplemented!();
    }

    /// Draws a sprite  to the screen.
    ///
    /// ```rust,no_run
    ///# extern crate lux;
    /// use std::path::Path;
    /// use lux::prelude::*;
    /// use lux::graphics::ColorVertex;
    ///# fn main() {
    ///
    /// let mut window = Window::new().unwrap();
    /// let mut frame = window.frame();
    /// let logo = window.load_texture_file(&Path::new("./logo.png"))
    ///                  .unwrap()
    ///                  .into_sprite();
    /// let (x, y) = (20.0, 50.0);
    /// frame.sprite(&logo, x, y).draw();
    ///# }
    /// ```
    fn sprite(&mut self, sprite: &Sprite, x: Float, y: Float) -> ContainedSprite<Self> {
        ContainedSprite {
            fields: BasicFields::new((x, y), sprite.ideal_size(), self, [1.0, 1.0, 1.0, 1.0]),
            sprite: sprite.clone()
        }
    }
}

impl <'a, C: 'a> BasicFields<'a, C> {
    fn new(pos: (Float, Float), size: (Float, Float), c: &'a mut C, color: [Float; 4]) -> BasicFields<'a, C> {
        BasicFields {
            fill_color: color,
            stroke_color: None,
            border: 0.0,
            transform: vecmath::mat4_id(),

            pos: pos,
            size: size,
            canvas: c
        }
    }
}

impl <'a, C> Ellipse<'a, C> {
    fn new(c: &'a mut C, pos: (Float, Float), size: (Float, Float), color: [Float; 4]) -> Ellipse<'a, C> {
        Ellipse {
            fields: BasicFields::new(pos, size, c, color),
            spokes: 90
        }
    }

    /// Sets the number of segments that are used to approximate a circle.
    ///
    /// ### Example
    /// ```rust,no_run
    ///# extern crate lux;
    /// use lux::prelude::*;
    /// use lux::graphics::ColorVertex;
    ///
    ///# fn main() {
    /// let mut window = Window::new().unwrap();
    /// let mut frame = window.frame();
    /// let (x, y, size) = (10.0, 10.0, 50.0);
    /// // draw a pentagon
    /// frame.circle(x, y, size).spokes(5).draw();
    ///# }
    /// ```
    pub fn spokes(&mut self, spokes: u16) -> &mut Self {
        self.spokes = spokes;
        self
    }
}

impl <'a, C> Rectangle<'a, C> {
    fn new(c: &'a mut C, pos: (Float, Float), size: (Float, Float), color: [Float; 4]) -> Rectangle<'a, C> {
        Rectangle {
            fields: BasicFields::new(pos, size, c, color),
        }
    }
}

impl <'a, C> Transform for Rectangle<'a, C> {
    fn current_matrix(&self) -> &[[Float; 4]; 4] {
        &self.fields.transform
    }
    fn current_matrix_mut(&mut self) -> &mut[[Float; 4]; 4] {
        &mut self.fields.transform
    }
}

impl <'a, C> Transform for Ellipse<'a, C> {
    fn current_matrix(&self) -> &[[Float; 4]; 4] {
        &self.fields.transform
    }
    fn current_matrix_mut(&mut self) -> &mut[[Float; 4]; 4] {
        &mut self.fields.transform
    }
}

impl <'a, C> Colored for Ellipse<'a, C> {
    fn get_color(&self) -> [Float; 4] {
        self.fields.fill_color
    }

    fn color<A: Color>(&mut self, color: A) -> &mut Self {
        self.fields.fill_color = color.to_rgba();
        self
    }
}

impl <'a, C> Colored for Rectangle<'a, C> {
    fn get_color(&self) -> [Float; 4] {
        self.fields.fill_color
    }

    fn color<A: Color>(&mut self, color: A) -> &mut Self{
        self.fields.fill_color = color.to_rgba();
        self
    }
}

impl <'a, C> Colored for ContainedSprite<'a, C> {
    fn get_color(&self) -> [Float; 4] {
        self.fields.fill_color
    }

    fn color<A: Color>(&mut self, color: A) -> &mut Self{
        self.fields.fill_color = color.to_rgba();
        self
    }
}

impl <'a, C> Ellipse<'a, C> where C: LuxCanvas + 'a {
    /// Fills in the ellipse with a solid color.
    pub fn fill(&mut self) {
        use std::f32::consts::PI;
        use num::traits::Float as Nfloat;

        let color = self.get_color();
        let spokes = self.spokes;
        let mut vertices = vec![];

        let mut theta = 0.0;
        while theta <= 2.0 * PI {
            let p = [theta.sin(), theta.cos()];
            vertices.push(ColorVertex { pos: p, color: color });
            theta += (2.0 * PI) / (spokes as Float);
        }

        //let mut trx = vecmath::mat4_id();
        //trx.scale(0.5, 0.5);

        let mut transform = generate_transform(&self.fields);

        //trx = vecmath::col_mat4_mul(trx, transform);
        transform.translate(0.5, 0.5);
        transform.scale(0.5, 0.5);

        self.fields.canvas.draw_colored(TriangleFan,
                               &vertices[..],
                               None,
                               Some(transform)).unwrap()
    }
}

fn generate_transform<'a, C>(fields: &BasicFields<'a, C>) -> [[Float; 4]; 4] {
        let (x, y) = fields.pos;
        let (mut sx, mut sy) = fields.size;
        sx -= fields.border * 2.0;
        sy -= fields.border * 2.0;

        if sx < 0.0 { sx = 0.0 }
        if sy < 0.0 { sy = 0.0 }

        let mut trx = vecmath::mat4_id();
        trx.translate(x, y);
        let mut trx = vecmath::col_mat4_mul(trx, fields.transform);
        trx.translate(fields.border, fields.border);
        trx.scale(sx, sy);
        trx
}

impl <'a, C> ContainedSprite<'a, C> where C: LuxCanvas + 'a {
    /// Sets the side of the sprite when drawn to the screen.
    ///
    /// The default size is the "ideal size", that is, 1 pixel in the texture
    /// goes to 1 pixel on the screen.
    pub fn size(&mut self, w: Float, h: Float) -> &mut ContainedSprite<'a, C> {
        self.fields.size = (w, h);
        self
    }

    /// Draws the sprite to the screen.
    pub fn draw(&mut self) {
        let bounds = self.sprite.bounds();

        let top_left = bounds[0];
        let top_right = bounds[1];
        let bottom_left = bounds[2];
        let bottom_right = bounds[3];

        let tex_vs = vec![
            TexVertex {pos: [1.0, 0.0], tex_coords: top_right},
            TexVertex {pos: [0.0, 0.0], tex_coords: top_left},
            TexVertex {pos: [0.0, 1.0], tex_coords: bottom_left},
            TexVertex {pos: [1.0, 1.0], tex_coords: bottom_right},
        ];

        let idxs = [0u16, 1, 2, 0, 2, 3];

        let transform = generate_transform(&self.fields);

        self.fields.canvas.draw_tex(TrianglesList,
                      &tex_vs[..],
                      Some(&idxs[..]),
                      Some(transform),
                      self.sprite.texture(),
                      Some(self.fields.fill_color)).unwrap();
    }
}

impl <'a, C> Rectangle<'a, C> where C: LuxCanvas + 'a {
    /// Fills the rectangle with a solid color.
    pub fn fill(&mut self) {
        let color = self.get_color();
        let vertices = [
            ColorVertex{ pos: [1.0, 0.0], color: color },
            ColorVertex{ pos: [0.0, 0.0], color: color },
            ColorVertex{ pos: [0.0, 1.0], color: color },
            ColorVertex{ pos: [1.0, 1.0], color: color },
        ];

        let idxs = [0, 1, 2, 0, 2, 3];

        let transform = generate_transform(&self.fields);

        self.fields.canvas.draw_colored(TrianglesList,
                               &vertices[..], Some(&idxs[..]),
                               Some(transform)).unwrap();
    }

    /// Draws a border around the rectangle.
    pub fn stroke(&mut self) {
        let offset_pos = self.fields.pos;
        let size = self.fields.size;
        let border = self.fields.border;
        let transform = self.fields.transform;
        let color = self.fields.stroke_color.unwrap_or(self.get_color());

        self.fields.border = 0.0;

        self.fields.canvas.with_matrix(|canvas| {
            canvas.translate(offset_pos.0, offset_pos.1);
            canvas.apply_matrix(transform);
            canvas.with_color(color, |canvas| {
                // TOP
                canvas.rect(0.0, 0.0, size.0, border).fill();
                canvas.rect(0.0, size.1 - border, size.0, border).fill();
                canvas.rect(0.0, border, border, size.1 - border * 2.0).fill();
                canvas.rect(size.0 - border, border, border, size.1 - border * 2.0).fill();
            });
        });
    }

    /// Both fills and strokes the rectangle.
    pub fn fill_and_stroke(&mut self) {
        self.fill();
        self.stroke();
    }

    /// Sets the size of the border.  The border is drawn using the
    /// `stroke()` function.
    pub fn border<A: Color>(&mut self, border_size: Float, color: A) -> &mut Rectangle<'a, C> {
        self.fields.border = border_size;
        self.fields.stroke_color = Some(color.to_rgba());
        self
    }
}
