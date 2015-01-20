use vecmath;
use super::{
    LuxCanvas,
    ColorVertex,
    Color,
    Transform,
    Figure
};

struct BasicFields {
    fill_color: Option<[f32; 4]>,
    stroke_color: Option<[f32; 4]>,
    padding: (f32, f32, f32, f32),
    border: f32,
    transform: [[f32; 4]; 4]
}

/// An ellipse that can be drawn to the screen.
pub struct Ellipse {
    fields: BasicFields,
    pos: (f32, f32),
    size: (f32, f32),
    spokes: u8
}

/// A Rectangle that can be drawn to the screen.
pub struct Rectangle {
    fields: BasicFields,
    pos: (f32, f32),
    size: (f32, f32),
}

pub struct FilledEllipse {
    ellipse: Ellipse
}

pub struct StrokedEllipse {
    ellipse: Ellipse
}

pub struct FilledRectangle {
    rectangle: Rectangle
}

pub struct StrokedRectangle {
    rectangle: Rectangle
}

pub fn circle(pos: (f32, f32), size: f32) -> Ellipse {
    Ellipse::new(pos, (size, size))
}

pub fn ellipse(pos: (f32, f32), size: (f32, f32)) -> Ellipse {
    Ellipse::new(pos, size)
}

pub fn square(pos: (f32, f32), size: f32) -> Rectangle {
    Rectangle::new(pos, (size, size))
}

pub fn rect(pos: (f32, f32), size: (f32, f32)) -> Rectangle {
    Rectangle::new(pos, size)
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


impl Ellipse {
    fn new(pos: (f32, f32), size: (f32, f32)) -> Ellipse {
        Ellipse {
            fields: BasicFields::new(),
            pos: pos,
            size: size,
            spokes: 90
        }
    }
}

impl  Rectangle {
    fn new(pos: (f32, f32), size: (f32, f32)) -> Rectangle {
        Rectangle {
            fields: BasicFields::new(),
            pos: pos,
            size: size
        }
    }
}

impl Transform for Rectangle {
    fn current_matrix(&self) -> &[[f32; 4]; 4] {
        &self.fields.transform
    }
    fn current_matrix_mut(&mut self) -> &mut[[f32; 4]; 4] {
        &mut self.fields.transform
    }
}

impl Transform for Ellipse {
    fn current_matrix(&self) -> &[[f32; 4]; 4] {
        &self.fields.transform
    }
    fn current_matrix_mut(&mut self) -> &mut[[f32; 4]; 4] {
        &mut self.fields.transform
    }
}

impl Figure for Ellipse {
    fn draw<C: LuxCanvas>(&self, canvas: &mut C) {
        self.draw_fill(canvas);
        // TODO: self.stroke(canvas)
    }
}

impl Figure for FilledEllipse {
    fn draw<C: LuxCanvas>(&self, canvas: &mut C) {
        self.ellipse.draw_fill(canvas);
    }
}

impl Figure for StrokedEllipse {
    fn draw<C: LuxCanvas>(&self, canvas: &mut C) {
        // TODO: self.ellipse.draw_stroke(canvas);
    }
}

impl Ellipse {
    fn draw_fill<C: LuxCanvas>(&self, canvas: &mut C) {
        use std::f32::consts::PI;
        use std::num::Float;

        let color = self.fields.fill_color
                               .unwrap_or_else(|| *canvas.current_fill_color());
        let spokes = self.spokes;
        let mut vertices = vec![];

        let mut theta = 0.0;
        while theta <= 2.0 * PI {
            let p = [theta.sin(), theta.cos()];
            vertices.push(ColorVertex { pos: p, color: color });
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

        canvas.draw_shape(super::TriangleFan, &vertices[], None, Some(trx));

    }

    pub fn fill(self) -> FilledEllipse { FilledEllipse { ellipse: self }}
    pub fn stroke(self) -> StrokedEllipse { StrokedEllipse { ellipse: self }}

    pub fn fill_color<C: Color>(&mut self, c: C) -> &mut Ellipse {
        self.fields.fill_color = Some(c.to_rgba());
        self
    }

    pub fn stroke_color<C: Color>(&mut self, c: C) -> &mut Ellipse {
        self.fields.stroke_color = Some(c.to_rgba());
        self
    }

    /// Add padding to the ellipse.
    ///
    /// Padding causes the ellipse to be drawn
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
    pub fn padding<P: Padding>(&mut self, padding: P) -> &mut Ellipse {
        self.fields.padding = padding.as_padding();
        self
    }
}
impl Figure for Rectangle {
    fn draw<C: LuxCanvas>(&self, canvas: &mut C) {
        self.draw_fill(canvas);
        // TODO: self.stroke(canvas)
    }
}

impl Figure for FilledRectangle {
    fn draw<C: LuxCanvas>(&self, canvas: &mut C) {
        self.rectangle.draw_fill(canvas);
    }
}

impl Figure for StrokedRectangle {
    fn draw<C: LuxCanvas>(&self, canvas: &mut C) {
        // TODO: self.rectangle.draw_stroke(canvas);
    }
}


impl  Rectangle {
    /// Fills the rectangle with a solid color.
    pub fn draw_fill<C: LuxCanvas>(&self, canvas: &mut C) {
        let color = self.fields.fill_color
                               .unwrap_or_else(|| *canvas.current_fill_color());
        let vertices = [
                ColorVertex{ pos: [1.0, 0.0], color: color },
                ColorVertex{ pos: [0.0, 0.0], color: color },
                ColorVertex{ pos: [0.0, 1.0], color: color },
                ColorVertex{ pos: [1.0, 1.0], color: color },
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

        let mut transform = vecmath::col_mat4_mul(local, self.fields.transform);
        transform.scale(sx, sy);

        canvas.draw_shape(super::TrianglesList,
                               &vertices[], Some(&idxs[]),
                               Some(transform));
    }

    /// Draws a border around the rectangle.
    pub fn stroke(self) -> StrokedRectangle { StrokedRectangle{rectangle: self }}
    pub fn fill(self) -> FilledRectangle { FilledRectangle{rectangle: self }}

    /// Sets the size of the border.  The border is drawn using the
    /// `stroke()` function.
    pub fn border(&mut self, border_size: f32) -> &mut Rectangle {
        self.fields.border = border_size;
        self
    }

    pub fn fill_color<C: Color>(&mut self, c: C) -> &mut Rectangle {
        self.fields.fill_color = Some(c.to_rgba());
        self
    }

    pub fn stroke_color<C: Color>(&mut self, c: C) -> &mut Rectangle {
        self.fields.stroke_color = Some(c.to_rgba());
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
    pub fn padding<P: Padding>(&mut self, padding: P) -> &mut Rectangle {
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
