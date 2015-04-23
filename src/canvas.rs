use super::primitive_canvas::PrimitiveCanvas;
use super::prelude::{
    ColorVertex,
    TexVertex,
    TriangleFan,
    TrianglesList,
    Colored,
    Color,
    Points,
    Sprite,
    Transform
};

use vecmath;

struct BasicFields<'a, C: 'a> {
    fill_color: [f32; 4],
    stroke_color: Option<[f32; 4]>,
    padding: (f32, f32, f32, f32),
    border: f32,
    transform: [[f32; 4]; 4],

    pos: (f32, f32),
    size: (f32, f32),
    canvas: &'a mut C
}

/// An ellipse that can be drawn to the screen.
#[must_use = "shapes contain context, and must be drawn with `fill()`, `stroke()`, or `fill_stroke()`"]
pub struct Ellipse<'a, C: 'a> {
    fields: BasicFields<'a, C>,
    spokes: u8
}

/// A Rectangle that can be drawn to the screen.
#[must_use = "shapes contain context, and must be drawn with `fill()`, `stroke()`, or `fill_stroke()`"]
pub struct Rectangle<'a, C: 'a> {
    fields: BasicFields<'a, C>,
}

#[must_use = "sprite references contain context, and must be drawn with `draw()`"]
pub struct ContainedSprite<'a, C: 'a>  {
    fields: BasicFields<'a, C>,
    sprite: Sprite
}

/// LuxCanvas is the main trait for drawing in Lux.  It supports all operations
/// that paint to the screen or to a buffer.
pub trait LuxCanvas: PrimitiveCanvas + Colored + Sized + Transform {
    /// Returns the size of the canvas as a pair of (width, height).
    fn size(&self) -> (f32, f32);

    /// Returns the width of the canvas.
    fn width(&self) -> f32 {
        match self.size() {
            (w, _) => w
        }
    }

    /// Returns the height of the canvas.
    fn height(&self) -> f32 {
        match self.size() {
            (_, h) => h
        }
    }

    fn clear<C: Color>(&mut self, color: C) {
        PrimitiveCanvas::clear(self, color);
    }

    /// Returns a rectangle with the given dimensions and position.
    fn rect<'a>(&'a mut self, x: f32, y: f32, w: f32, h: f32) -> Rectangle<'a, Self> {
        let c = self.color();
        Rectangle::new(self, (x, y), (w, h), c)
    }

    /// Returns a square with the given dimensions and position.
    fn square<'a>(&'a mut self, x: f32, y: f32, size: f32) -> Rectangle<'a, Self> {
        let c = self.color();
        Rectangle::new(self, (x, y), (size, size), c)
    }

    /// Returns an ellipse with the given dimensions and position.
    fn ellipse<'a>(&'a mut self, x: f32, y: f32, w: f32, h: f32) -> Ellipse<'a, Self> {
        let c = self.color();
        Ellipse::new(self, (x, y), (w, h), c)
    }

    /// Returns an circle with the given dimensions and position.
    fn circle<'a>(&'a mut self, x: f32, y: f32, size: f32) -> Ellipse<'a, Self> {
        let c = self.color();
        Ellipse::new(self, (x, y), (size, size), c)
    }

    // TODO: unify this and draw_pixels.
    fn draw_pixel<C: Color>(&mut self, x: f32, y: f32, color: C) {
        let vertex = ColorVertex {
            pos: [x, y],
            color: color.to_rgba(),
        };
        self.draw_shape(Points, &[vertex][..], None, None);
    }

    fn draw_pixels<C: Color, I: Iterator<Item = ((f32, f32), C)>>(&mut self, pixels: I) {
        let v: Vec<_> = pixels
            .map(|((px, py), c)|{
                ColorVertex {
                    pos: [px + 0.5, py + 0.5],
                    color: c.to_rgba(),
                }
            }) .collect();
        self.draw_shape(Points, &v[..], None, None);
    }

    /// Draws a single line from `start` to `end` with a
    /// thickness of `line_size`.
    fn draw_line(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32, _line_size: f32) {
        unimplemented!();
    }

    /// Draws a series of lines from each point to the next with a thickness
    /// of `line_size`.
    fn draw_lines<I: Iterator<Item = (f32, f32)>>(&mut self, mut _positions: I, _line_size: f32) {
        unimplemented!();
    }

    /// Draws an arc centered at `pos` from `angle1` to `angle_2` with a
    /// thickness of `line_size`.
    fn draw_arc(&mut self, _pos: (f32, f32), _radius: f32, _angle1: f32,
                _angle2: f32, _line_size: f32) {
        unimplemented!();
    }

    /// Draws a sprite  to the screen.
    fn sprite(&mut self, sprite: &Sprite, x: f32, y: f32) -> ContainedSprite<Self> {
        ContainedSprite {
            fields: BasicFields::new((x, y), sprite.ideal_size(), self, [1.0, 1.0, 1.0, 1.0]),
            sprite: sprite.clone()
        }
    }
}

impl <'a, C: 'a> BasicFields<'a, C> {
    fn new(pos: (f32, f32), size: (f32, f32), c: &'a mut C, color: [f32; 4]) -> BasicFields<'a, C> {
        BasicFields {
            fill_color: color,
            stroke_color: None,
            padding: (0.0, 0.0, 0.0, 0.0),
            border: 0.0,
            transform: vecmath::mat4_id(),

            pos: pos,
            size: size,
            canvas: c
        }
    }
}

impl <'a, C> Ellipse<'a, C> {
    fn new(c: &'a mut C, pos: (f32, f32), size: (f32, f32), color: [f32; 4]) -> Ellipse<'a, C> {
        Ellipse {
            fields: BasicFields::new(pos, size, c, color),
            spokes: 90
        }
    }
}

impl <'a, C> Rectangle<'a, C> {
    fn new(c: &'a mut C, pos: (f32, f32), size: (f32, f32), color: [f32; 4]) -> Rectangle<'a, C> {
        Rectangle {
            fields: BasicFields::new(pos, size, c, color),
        }
    }
}

impl <'a, C> Transform for Rectangle<'a, C> {
    fn current_matrix(&self) -> &[[f32; 4]; 4] {
        &self.fields.transform
    }
    fn current_matrix_mut(&mut self) -> &mut[[f32; 4]; 4] {
        &mut self.fields.transform
    }
}

impl <'a, C> Colored for Ellipse<'a, C> {
    fn color(&self) -> [f32; 4] {
        self.fields.fill_color
    }

    fn set_color<A: Color>(&mut self, color: A) -> &mut Self {
        self.fields.fill_color = color.to_rgba();
        self
    }
}

impl <'a, C> Colored for Rectangle<'a, C> {
    fn color(&self) -> [f32; 4] {
        self.fields.fill_color
    }

    fn set_color<A: Color>(&mut self, color: A) -> &mut Self{
        self.fields.fill_color = color.to_rgba();
        self
    }
}

impl <'a, C> Colored for ContainedSprite<'a, C> {
    fn color(&self) -> [f32; 4] {
        self.fields.fill_color
    }

    fn set_color<A: Color>(&mut self, color: A) -> &mut Self{
        self.fields.fill_color = color.to_rgba();
        self
    }
}

impl <'a, C> Ellipse<'a, C> where C: LuxCanvas + 'a {
    /// Fills in the ellipse with a solid color.
    pub fn fill(&mut self) {
        use std::f32::consts::PI;
        use num::traits::Float;

        let color = self.color();
        let spokes = self.spokes;
        let mut vertices = vec![];

        let mut theta = 0.0;
        while theta <= 2.0 * PI {
            let p = [theta.sin(), theta.cos()];
            vertices.push(ColorVertex { pos: p, color: color });
            theta += (2.0 * PI) / (spokes as f32);
        }

        let (mut x, mut y) = self.fields.pos;
        x += self.fields.border + self.fields.padding.0;
        y += self.fields.border + self.fields.padding.2;

        let (mut sx, mut sy) = self.fields.size;
        sx -= self.fields.border + self.fields.padding.0 + self.fields.padding.1;
        sy -= self.fields.border + self.fields.padding.2 + self.fields.padding.3;
        sx /= 2.0;
        sy /= 2.0;

        let mut trx = vecmath::mat4_id();
        trx.translate(x + sx, y + sy);
        trx.scale(sx, sy);
        trx = vecmath::col_mat4_mul(trx, self.fields.transform);

        self.fields.canvas.draw_shape(TriangleFan,
                               &vertices[..],
                               None,
                               Some(trx));
    }
    /*
    pub fn padding<P: Padding>(&mut self, padding: P) -> &mut Ellipse<'a, C> {
        self.fields.padding = padding.as_padding();
        self
    }*/
}

fn generate_transform<'a, C>(fields: &BasicFields<'a, C>) -> [[f32; 4]; 4] {
        let (x, y) = fields.pos;
//        x += fields.border + fields.padding.0;
//        y += fields.border + fields.padding.2;

        let (mut sx, mut sy) = fields.size;
        sx -= fields.border * 2.0; // + fields.padding.0 + fields.padding.1;
        sy -= fields.border * 2.0; // + fields.padding.2 + fields.padding.3;

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
    pub fn size(&mut self, w: f32, h: f32) -> &mut ContainedSprite<'a, C> {
        self.fields.size = (w, h);
        self
    }

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

        let idxs = [0u32, 1, 2, 0, 2, 3];

        let transform = generate_transform(&self.fields);

        self.fields.canvas.draw_tex(TrianglesList,
                      &tex_vs[..],
                      Some(&idxs[..]),
                      Some(transform),
                      self.sprite.texture(),
                      Some(self.fields.fill_color));
    }
}

impl <'a, C> Rectangle<'a, C> where C: LuxCanvas + 'a {
    /// Fills the rectangle with a solid color.
    pub fn fill(&mut self) {
        let color = self.color();
        let vertices = [
            ColorVertex{ pos: [1.0, 0.0], color: color },
            ColorVertex{ pos: [0.0, 0.0], color: color },
            ColorVertex{ pos: [0.0, 1.0], color: color },
            ColorVertex{ pos: [1.0, 1.0], color: color },
        ];

        let idxs = [0, 1, 2, 0, 2, 3];

        let transform = generate_transform(&self.fields);

        self.fields.canvas.draw_shape(TrianglesList,
                               &vertices[..], Some(&idxs[..]),
                               Some(transform));
    }

    /// Draws a border around the rectangle.
    pub fn stroke(&mut self) -> &mut Rectangle<'a, C> {
        let offset_pos = self.fields.pos;
        let size = self.fields.size;
        let border = self.fields.border;
        let transform = self.fields.transform;
        let color = self.fields.stroke_color.unwrap_or(self.color());

        self.fields.border = 0.0;

        self.fields.canvas.with_matrix(|canvas| {
            canvas.translate(offset_pos.0, offset_pos.1);
            canvas.apply_matrix(transform);
            canvas.with_color(color, |canvas| {
                // TOP
                canvas.rect(0.0, 0.0, size.0, border)
                      .fill();
                canvas.rect(0.0, size.1 - border, size.0, border)
                      .fill();
                canvas.rect(0.0, border, border, size.1 - border * 2.0)
                      .fill();
                canvas.rect(size.0 - border, border, border, size.1 - border * 2.0)
                      .fill();
            });
        });
        self
    }

    pub fn fill_and_stroke(&mut self) {
        self.fill();
        self.stroke();
    }

    /// Sets the size of the border.  The border is drawn using the
    /// `stroke()` function.
    pub fn border<A: Color>(&mut self, border_size: f32, color: A) -> &mut Rectangle<'a, C> {
        self.fields.border = border_size;
        self.fields.stroke_color = Some(color.to_rgba());
        self
    }
}
