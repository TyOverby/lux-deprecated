use super::{
    Colored,
    Transform,
    StackedTransform,
    Vertex
};

use vecmath;

struct BasicFields {
    fill_color: Option<[f32, ..4]>,
    stroke_color: Option<[f32, ..4]>,
    padding: (f32, f32, f32, f32),
    border: f32,
    transform: [[f32, ..4], ..4]
}

/// An ellipse that can be drawn to the screen.
pub struct Ellipse<'a, C: 'a> {
    fields: BasicFields,
    canvas: &'a mut C,
    pos: (f32, f32),
    size: (f32, f32),
    spokes: u8
}

/// A Rectangle that can be drawn to the screen.
pub struct Rectangle<'a, C: 'a> {
    fields: BasicFields,
    pos: (f32, f32),
    size: (f32, f32),
    canvas: &'a mut C
}

/// A primitive canvas is a canvas that can be drawn to with only the
/// `draw_shape` function.
pub trait PrimitiveCanvas {
    /// typ: The primitive type used to draw the vertices.
    /// vs : A slice of vertices to be drawn.
    /// idxs: An optional list of indices that can be used to index into
    ///       the vertex array.  Useful if you have many points that are
    ///       duplicates of each other.
    /// mat: An optional transformation matrix that would be applied to the
    ///      each point before drawing.
    fn draw_shape(&mut self,
                  typ: super::PrimitiveType,
                  vs: &[super::Vertex],
                  idxs: Option<&[u32]>,
                  mat: Option<[[f32, ..4], ..4]>);
}

/// LuxCanvas is the main trait for drawing in Lux.  It supports all operations
/// that paint to the screen or to a buffer.
pub trait LuxCanvas: Transform + StackedTransform + PrimitiveCanvas  + Colored {
    /// Returns the size of the canvas as a pair of (width, height).
    fn size(&self) -> (u32, u32);

    /// Returns the width of the canvas.
    fn width(&self) -> u32 {
        match self.size() {
            (w, _) => w
        }
    }

    /// Returns the height of the canvas.
    fn height(&self) -> u32 {
        match self.size() {
            (_, h) => h
        }
    }

    /// Returns a rectangle with the given dimensions and position.
    fn rect<'a>(&'a mut self, pos: (f32, f32), size: (f32, f32)) -> Rectangle<'a, Self> {
        Rectangle::new(self, pos, size)
    }

    /// Returns a square with the given dimensions and position.
    fn square<'a>(&'a mut self, pos: (f32, f32), size: f32) -> Rectangle<'a, Self> {
        Rectangle::new(self, pos, (size, size))
    }

    /// Returns an ellipse with the given dimensions and position.
    fn ellipse<'a>(&'a mut self, pos: (f32, f32), size: (f32, f32)) -> Ellipse<'a, Self> {
        Ellipse::new(self, pos, size)
    }

    /// Returns an circle with the given dimensions and position.
    fn circle<'a>(&'a mut self, pos: (f32, f32), size: f32) -> Ellipse<'a, Self> {
        Ellipse::new(self, pos, (size, size))
    }

    /// Draws a single line from `start` to `end` with a
    /// thickness of `line_size`.
    fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), line_size: f32);

    /// Draws a series of lines from each point to the next with a thickness
    /// of `line_size`.
    fn draw_lines<I: Iterator<(f32, f32)>>(&mut self, mut positions: I, line_size: f32);

    /// Draws an arc centered at `pos` from `angle1` to `angle_2` with a
    /// thickness of `line_size`.
    fn draw_arc(&mut self, pos: (f32, f32), radius: f32, angle1: f32,
                angle2: f32, line_size: f32);

    /// Draws text to the screen.
    fn draw_text(&mut self, pos: (f32, f32), text: &str);
}

impl BasicFields {
    fn new() -> BasicFields {
        BasicFields {
            fill_color: None,
            stroke_color: None,
            padding: (0.0, 0.0, 0.0, 0.0),
            border: 0.0,
            transform: vecmath::mat4_id(),
        }
    }
}

impl <'a, C> Ellipse<'a, C> {
    fn new(c: &'a mut C, pos: (f32, f32), size: (f32, f32)) -> Ellipse<'a, C> {
        Ellipse {
            fields: BasicFields::new(),
            canvas: c,
            pos: pos,
            size: size,
            spokes: 90
        }
    }
}

impl <'a, C> Rectangle<'a, C> {
    fn new(c: &'a mut C, pos: (f32, f32), size: (f32, f32)) -> Rectangle<'a, C> {
        Rectangle {
            fields: BasicFields::new(),
            canvas: c,
            pos: pos,
            size: size
        }
    }
}

impl <'a, C> Transform for Rectangle<'a, C> {
    fn current_matrix(&self) -> &[[f32, ..4], ..4] {
        &self.fields.transform
    }
    fn current_matrix_mut(&mut self) -> &mut[[f32, ..4], ..4] {
        &mut self.fields.transform
    }
}

impl <'a, C> Colored for Ellipse<'a, C> where C: Colored {
    fn current_fill_color(&self) -> &[f32, ..4] {
        self.fields.fill_color.as_ref().unwrap_or_else(
            || self.canvas.current_fill_color())
    }
    fn current_fill_color_mut(&mut self) -> &mut[f32, ..4] {
        if self.fields.fill_color.is_none() {
            self.fields.fill_color = Some(*self.canvas.current_fill_color());
        }
        self.fields.fill_color.as_mut().unwrap()
    }

    fn current_stroke_color(&self) -> &[f32, ..4] {
        self.fields.stroke_color.as_ref().unwrap_or_else(
            || self.canvas.current_stroke_color())
    }
    fn current_stroke_color_mut(&mut self) -> &mut[f32, ..4] {
        if self.fields.stroke_color.is_none() {
            self.fields.stroke_color = Some(*self.canvas.current_stroke_color());
        }
        self.fields.stroke_color.as_mut().unwrap()
    }
}

impl <'a, C> Colored for Rectangle<'a, C> where C: Colored {
    fn current_fill_color(&self) -> &[f32, ..4] {
        self.fields.fill_color.as_ref().unwrap_or_else(
            || self.canvas.current_fill_color())
    }
    fn current_fill_color_mut(&mut self) -> &mut[f32, ..4] {
        if self.fields.fill_color.is_none() {
            self.fields.fill_color = Some(*self.canvas.current_fill_color());
        }
        self.fields.fill_color.as_mut().unwrap()
    }

    fn current_stroke_color(&self) -> &[f32, ..4] {
        self.fields.stroke_color.as_ref().unwrap_or_else(
            || self.canvas.current_stroke_color())
    }
    fn current_stroke_color_mut(&mut self) -> &mut[f32, ..4] {
        if self.fields.stroke_color.is_none() {
            self.fields.stroke_color = Some(*self.canvas.current_stroke_color());
        }
        self.fields.stroke_color.as_mut().unwrap()
    }
}

impl <'a, C> Ellipse<'a, C>
where C: LuxCanvas + PrimitiveCanvas + 'a {

    /// Fills in the ellipse with a solid color.
    pub fn fill(&mut self) -> &mut Ellipse<'a, C> {
        use std::f32::consts::PI;
        use std::num::FloatMath;

        let color = *self.current_fill_color();
        let spokes = self.spokes;
        let mut vertices = vec![];

        let mut theta = 0.0;
        while theta <= 2.0 * PI {
            let p = [theta.sin(), theta.cos()];
            vertices.push(Vertex { pos: p, tex: p, color: color });
            theta += (2.0 * PI) / (spokes as f32);
        }

        let (mut x, mut y) = self.pos;
        x += self.fields.border + self.fields.padding.0;
        y += self.fields.border + self.fields.padding.2;

        let (mut sx, mut sy) = self.size;
        sx -= self.fields.border + self.fields.padding.0 + self.fields.padding.1;
        sy -= self.fields.border + self.fields.padding.2 + self.fields.padding.3;
        sx /= 2.0;
        sy /= 2.0;

        let mut trx = vecmath::mat4_id();
        trx.translate(x + sx, y + sy);
        trx.scale(sx, sy);
        trx = vecmath::col_mat4_mul(trx, self.fields.transform);

        self.canvas.draw_shape(super::TriangleFan,
                               vertices.as_slice(),
                               None,
                               Some(trx));
        self
    }

    /// Add padding to the ellipse.  Padding causes the ellipse to be drawn
    /// constrained to the original bounding dimensions with the additional
    /// constraints of the padding.
    ///
    /// Example:
    /// ```
    /// let padd = 5.0;
    /// lux.circle((pos.0 + padd, pos.1 + padd), (size.0 - 2.0 * padd, size.1 - 2.0 * \
    /// padd)).fill();
    /// lux.circle(pos, size).padding(padd).fill(); // equivalant
    /// ```
    pub fn padding<P: Padding>(&mut self, padding: P) -> &mut Ellipse<'a, C> {
        self.fields.padding = padding.as_padding();
        self
    }
}

impl <'a, C> Rectangle<'a, C>
where C: LuxCanvas + PrimitiveCanvas + 'a {
    /// Fills the rectangle with a solid color.
    pub fn fill(&mut self) -> &mut Rectangle<'a, C> {
        let color = *self.current_fill_color();
        let vertices = [
                Vertex{ pos: [1.0, 0.0], tex: [1.0, 0.0], color: color },
                Vertex{ pos: [0.0, 0.0], tex: [0.0, 0.0], color: color },
                Vertex{ pos: [0.0, 1.0], tex: [0.0, 1.0], color: color },
                Vertex{ pos: [1.0, 1.0], tex: [1.0, 1.0], color: color },
        ];
        let idxs = [0, 1, 2, 0, 2, 3];

        let (mut x, mut y) = self.pos;
        x += self.fields.border + self.fields.padding.0;
        y += self.fields.border + self.fields.padding.2;

        let (mut sx, mut sy) = self.size;
        sx -= self.fields.border + self.fields.padding.0 + self.fields.padding.1;
        sy -= self.fields.border + self.fields.padding.2 + self.fields.padding.3;

        let mut local = vecmath::mat4_id();
        local.translate(x, y);

        //let transform = vecmath::col_mat4_mul(self.fields.transform, local);
        let mut transform = vecmath::col_mat4_mul(local, self.fields.transform);
        transform.scale(sx, sy);

        self.canvas.draw_shape(super::TriangleList,
                               vertices.as_slice(), Some(idxs.as_slice()),
                               Some(transform));
        self
    }

    /// Draws a border around the rectangle.
    pub fn stroke(&mut self) -> &mut Rectangle<'a, C> {
        self
    }

    /// Sets the size of the border.  The border is drawn using the
    /// `stroke()` function.
    pub fn border(&mut self, border_size: f32) -> &mut Rectangle<'a, C> {
        self.fields.border = border_size;
        self
    }

    /// Add padding to the rectangle.
    /// Padding causes the rectangleto be drawn
    /// constrained to the original bounding dimensions with the additional
    /// constraints of the padding.
    ///
    /// Example:
    /// ```
    /// let padd = 5.0;
    /// lux.rect((pos.0 + padd, pos.1 + padd), (size.0 - 2.0 * padd, size.1 - 2.0 *
    /// padd)).fill();
    /// lux.rect(pos, size).padding(padd).fill(); // equivalant
    /// ```
    pub fn padding<P: Padding>(&mut self, padding: P) -> &mut Rectangle<'a, C> {
        self.fields.padding = padding.as_padding();
        self
    }
}

/// Padding can either be /// f32, (f32, f32), or (f32, f32, f32, f32)
/// where these values correspond
/// to "global", "horizontal, vertical " and "left, right, top, bottom".
pub trait Padding {
    /// -> (left, right, top, bottom)
    fn as_padding(self) -> (f32, f32, f32, f32);
}

impl Padding for f32 {
    fn as_padding(self) -> (f32, f32, f32, f32) {
        (self, self, self, self)
    }
}

impl Padding for (f32, f32) {
    fn as_padding(self) -> (f32, f32, f32, f32) {
        let (h, v) = self;
        (h, h, v, v)
    }
}

impl Padding for (f32, f32, f32, f32) {
    fn as_padding(self) -> (f32, f32, f32, f32) {
        self
    }
}
