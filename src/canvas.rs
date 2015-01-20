use std::rc::Rc;

use super::{
    Colored,
    Figure,
    Sprite,
    Color,
    Transform,
    StackedTransform,
    ColorVertex,
    TexVertex,
};

use vecmath;
use glium;


/// A primitive canvas is a canvas that can be drawn to with only the
/// `draw_shape` function.
pub trait PrimitiveCanvas {
    /// Draws the verteces to the canvas. This function uses caching to
    /// batch draw calls that are similar.
    ///
    /// typ: The primitive type used to draw the vertices.
    /// vs : A slice of vertices to be drawn.
    /// idxs: An optional list of indices that can be used to index into
    ///       the ColorVertex array.  Useful if you have many points that are
    ///       duplicates of each other.
    /// mat: An optional transformation matrix that would be applied to the
    ///      each point before drawing.
    fn draw_shape(&mut self,
                  typ: super::PrimitiveType,
                  vs: &[ColorVertex],
                  idxs: Option<&[u32]>,
                  mat: Option<[[f32; 4]; 4]>);

    /// Flush all stored draw calls to the screen.
    fn flush_draw(&mut self);

    fn draw_shape_no_batch(&mut self,
                           typ: super::PrimitiveType,
                           vs: Vec<ColorVertex>,
                           idxs: Option<Vec<u32>>,
                           mat: Option<[[f32; 4]; 4]>);

    fn draw_tex(&mut self,
                typ: super::PrimitiveType,
                vs: &[TexVertex],
                idxs: Option<&[u32]>,
                mat: Option<[[f32; 4]; 4]>,
                Rc<glium::texture::Texture2d>);

    fn draw_tex_no_batch(&mut self,
                         typ: super::PrimitiveType,
                         vs: Vec<TexVertex>,
                         idxs: Option<Vec<u32>>,
                         mat: Option<[[f32; 4]; 4]>,
                         &glium::texture::Texture2d);
}

/// LuxCanvas is the main trait for drawing in Lux.  It supports all operations
/// that paint to the screen or to a buffer.
pub trait LuxCanvas: Transform + StackedTransform + PrimitiveCanvas + Colored + Sized {
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

    fn draw_pixel<C: Color>(&mut self, pos: (f32, f32), color: C) {
        let vertex = ColorVertex {
            pos: [pos.0, pos.1],
            color: color.to_rgba(),
        };
        self.draw_shape(super::Points, &[vertex][], None, None);
    }

    fn draw_pixels<C: Color, I: Iterator<Item = ((f32, f32), C)>>(&mut self, pixels: I) {
        let v: Vec<_> = pixels
            .map(|((px, py), c)|{
                ColorVertex {
                    pos: [px + 0.5, py + 0.5],
                    color: c.to_rgba(),
                }
            }) .collect();
        self.draw_shape(super::Points, &v[], None, None);
    }

    /// Draws a single line from `start` to `end` with a
    /// thickness of `line_size`.
    fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), line_size: f32);

    /// Draws a series of lines from each point to the next with a thickness
    /// of `line_size`.
    fn draw_lines<I: Iterator<Item = (f32, f32)>>(&mut self, mut positions: I, line_size: f32);

    /// Draws an arc centered at `pos` from `angle1` to `angle_2` with a
    /// thickness of `line_size`.
    fn draw_arc(&mut self, pos: (f32, f32), radius: f32, angle1: f32,
                angle2: f32, line_size: f32);

    /// Draws text to the screen.
    fn draw_text(&mut self, pos: (f32, f32), text: &str);

    fn draw_sprite(&mut self, sprite: &Sprite, pos: (f32, f32), size: (f32, f32)) {
        self.with_matrix(|slf| {
            slf.translate(pos.0, pos.1);
            slf.scale(size.0, size.1);
            sprite.draw(slf);
        })
    }

    fn draw<F: Figure>(&mut self, figure: &F) {
        figure.draw(self);
    }
}

