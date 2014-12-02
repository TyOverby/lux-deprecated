use super::Drawable;
use super::Color;
use super::LuxRaw;

pub trait LuxCanvas: LuxRaw {
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


    fn draw_rect(&mut self, pos: (f32, f32), size: (f32, f32));
    fn draw_border_rect(&mut self, pos: (f32, f32), size: (f32, f32), border_size: f32);

    fn draw_circle(&mut self, pos: (f32, f32), radius: f32);
    fn draw_border_circle(&mut self, pos: (f32, f32), radius: f32, border_size: f32);

    fn draw_ellipse(&mut self, pos: (f32, f32), size: (f32, f32));
    fn draw_border_ellipse(&mut self, pos: (f32, f32), size: (f32, f32), border_size: f32);

    fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), line_size: f32);
    fn draw_lines<I: Iterator<(f32, f32)>>(&mut self, mut positions: I, line_size: f32);
    fn draw_arc(&mut self, pos: (f32, f32), radius: f32, angle1: f32, angle2: f32);

    fn with_color<C: Color>(&mut self, color: C, f: |&mut Self| -> ());
    fn with_border_color<C: Color>(&mut self, color: C, f: |&mut Self| -> ());

    fn with_rotation(&mut self, rotation: f32, f: |&mut Self| -> ()) {
        self.push_matrix();
        self.rotate(rotation);
        f(self);
        self.pop_matrix();
    }
    fn with_translate(&mut self, dx: f32, dy: f32, f: |&mut Self| -> ()) {
        self.push_matrix();
        self.translate(dx, dy);
        f(self);
        self.pop_matrix();
    }
    fn with_scale(&mut self, scale_x: f32, scale_y: f32, f: |&mut Self| -> ()) {
        self.push_matrix();
        self.scale(scale_x, scale_y);
        f(self);
        self.pop_matrix();
    }
    fn with_shear(&mut self, sx: f32, sy: f32, f: |&mut Self| -> ()) {
        self.push_matrix();
        self.shear(sx, sy);
        f(self);
        self.pop_matrix();
    }

    fn draw<T: Drawable>(&mut self, figure: &T) {
        figure.draw(self);
    }

    fn draw_text(&mut self, pos: (f32, f32), text: &str);
}
