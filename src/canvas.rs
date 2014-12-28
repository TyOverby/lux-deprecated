use super::Drawable;
use super::Colored;
use super::Transform;
use super::StackedTransform;
use super::Vertex;
use vecmath;

struct BasicFields {
    fill_color: Option<[f32, ..4]>,
    stroke_color: Option<[f32, ..4]>,
    padding: (f32, f32, f32, f32),
    border: f32,
    transform: [[f32, ..4], ..4]
}

pub struct Ellipse<'a, C: 'a> {
    fields: BasicFields,
    canvas: &'a mut C,
    pos: (f32, f32),
    size: (f32, f32),
    spokes: u8
}

pub struct Rectangle<'a, C: 'a> {
    fields: BasicFields,
    pos: (f32, f32),
    size: (f32, f32),
    canvas: &'a mut C
}

pub trait PrimitiveCanvas {
    fn draw_shape(&mut self,
                  typ: super::PrimitiveType,
                  vs: &[super::Vertex],
                  idxs: Option<&[u32]>,
                  mat: Option<[[f32, ..4], ..4]>);
}

pub trait LuxCanvas: Transform + StackedTransform + PrimitiveCanvas  + Colored {
    fn size(&self) -> (u32, u32);
    fn width(&self) -> u32 {
        match self.size() {
            (w, _) => w
        }
    }

    fn height(&self) -> u32 {
        match self.size() {
            (_, h) => h
        }
    }

    fn rect<'a>(&'a mut self, pos: (f32, f32), size: (f32, f32)) -> Rectangle<'a, Self> {
        Rectangle::new(self, pos, size)
    }

    fn square<'a>(&'a mut self, pos: (f32, f32), size: f32) -> Rectangle<'a, Self> {
        Rectangle::new(self, pos, (size, size))
    }

    fn ellipse<'a>(&'a mut self, pos: (f32, f32), size: (f32, f32)) -> Ellipse<'a, Self> {
        Ellipse::new(self, pos, size)
    }

    fn circle<'a>(&'a mut self, pos: (f32, f32), size: f32) -> Ellipse<'a, Self> {
        Ellipse::new(self, pos, (size, size))
    }

    fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), line_size: f32);
    fn draw_lines<I: Iterator<(f32, f32)>>(&mut self, mut positions: I, line_size: f32);
    fn draw_arc(&mut self, pos: (f32, f32), radius: f32, angle1: f32, angle2: f32);

    fn draw<T: Drawable>(&mut self, figure: &T) {
        figure.draw(self);
    }

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

    pub fn padding<P: Padding>(&mut self, padding: P) -> &mut Ellipse<'a, C> {
        self.fields.padding = padding.as_padding();
        self
    }
}

impl <'a, C> Rectangle<'a, C>
where C: LuxCanvas + PrimitiveCanvas + 'a {
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

    pub fn stroke(&mut self) -> &mut Rectangle<'a, C> {
        self
    }

    pub fn border(&mut self, border_size: f32) -> &mut Rectangle<'a, C> {
        self.fields.border = border_size;
        self
    }

    /// Applies padding to the rectangle.  Padding can either be
    /// f32, (f32, f32), or (f32, f32, f32, f32) where these values correspond
    /// to "global", "horizontal, vertical " and "left, right, top, bottom"

    pub fn padding<P: Padding>(&mut self, padding: P) -> &mut Rectangle<'a, C> {
        self.fields.padding = padding.as_padding();
        self
    }
}

pub trait Padding {
    // (left, right, top, bottom)
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
