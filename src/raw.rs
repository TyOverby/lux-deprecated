use vecmath::{mat4_id, col_mat4_mul};
use super::Color;

/// A trait for objects that can be "transformed".  Transformations
/// include scaling, translation, shearing, rotating, and general
/// purpose matrix application.
pub trait Transform {
    /// Return a reference to the current matrix.
    fn current_matrix(&self) -> &[[f32, ..4], ..4];
    /// Return a mutible reference to the current matrix.
    fn current_matrix_mut(&mut self) -> &mut [[f32, ..4], ..4];

    /// Multiplies the current matrix against another.
    /// `self = self * other`.
    fn apply_matrix(&mut self, other: [[f32, ..4], ..4]) {
        let current = self.current_matrix_mut();
        *current = col_mat4_mul(*current, other);
    }

    /// Applies a translation transformation to the matrix.
    fn translate(&mut self, dx: f32, dy: f32) -> &mut Self {
        let mut prod = mat4_id();
        prod[3][0] = dx;
        prod[3][1] = dy;
        self.apply_matrix(prod);
        self
    }

    /// Applies a scaling transformation to the matrix.
    fn scale(&mut self, sx: f32, sy: f32) -> &mut Self {
        let mut prod = mat4_id();
        prod[0][0] = sx;
        prod[1][1] = sy;
        self.apply_matrix(prod);
        self
    }

    /// Applies a shearing transformation to the matrix.
    fn shear(&mut self, sx: f32, sy: f32) -> &mut Self {
        let mut prod = mat4_id();
        prod[1][0] = sx;
        prod[0][1] = sy;
        self.apply_matrix(prod);
        self
    }

    /// Applies a rotation transformation to the matrix.
    fn rotate(&mut self, theta: f32) -> &mut Self {
        use std::num::FloatMath;
        let mut prod = mat4_id();
        let (c, s) = (theta.cos(), theta.sin());
        prod[0][0] = c;
        prod[0][1] = s;
        prod[1][0] = -s;
        prod[1][1] = c;
        self.apply_matrix(prod);
        self
    }

    /// Combines rotation with translation to effectively
    /// rotate around a given point.
    fn rotate_around(&mut self, point: (f32, f32), theta: f32) -> &mut Self {
        self.rotate(theta);
        self.translate(-point.0, -point.1);
        self
    }
}

/// Turns out that implementing matrix transformations on matrices is a
/// no brainer!
impl Transform for [[f32, ..4], ..4] {
    fn current_matrix(&self) -> &[[f32, ..4], ..4] { self }
    fn current_matrix_mut(&mut self) -> &mut [[f32, ..4], ..4] { self }
}

/// Similar to opengl pushMatrix() and popMatrix(), this trait is
/// implemented on objects that will have a stack of operations
/// applied to it.
///
/// Instead of using push_matrix() and pop_matrix() manually, it is
/// advised that you use with_matrix() and provide a closure that operates
/// on a clean matrix for the duration of the closure.
pub trait StackedTransform: Transform {
    /// Pushes a clean matrix on to the stack that is a duplicate
    /// of the last matrix on the stack.
    fn push_matrix(&mut self);

    /// Pops the topmost matrix from the stack.
    fn pop_matrix(&mut self);

    /// Used when you want to make several successive calls to transformations
    /// on a single stacked matrix.
    ///
    /// Example:
    /// lux.with_matrix(|lux| {
    ///   lux.translate(5.0, 10.0);
    ///   lux.rotate(3.14 / 2.0);
    ///   lux.scale(2.0, 1.0);
    ///   // do other stuff
    /// });
    fn with_matrix<F>(&mut self, f: F) where F: FnOnce(&mut Self){
        self.push_matrix();
        f(self);
        self.pop_matrix();
    }
}

/// A trait representing objects that can be colored with
/// seperate fill colors and stroke colors.
///
/// The basic representation of a color is an array of 4 floats
/// where each value goes from 0.0 to 1.0 and is of the form
/// [r, b, b, a].
pub trait Colored {
    /// Returns a reference to the current fill color.
    fn current_fill_color(&self) -> &[f32, ..4];

    /// Returns a mutable reference to the current fill color.
    fn current_fill_color_mut(&mut self) -> &mut[f32, ..4];

    /// Returns a reference to the current stroke color.
    fn current_stroke_color(&self) -> &[f32, ..4];

    /// Returns a mutable reference to the current stroke color.
    fn current_stroke_color_mut(&mut self) -> &mut[f32, ..4];

    /// Sets the fill color.
    fn fill_color<C: Color>(&mut self, c: C) -> &mut Self {
        *self.current_fill_color_mut() = c.to_rgba();
        self
    }

    /// Sets the stroke color.
    fn stroke_color<C: Color>(&mut self, c: C) -> &mut Self {
        *self.current_stroke_color_mut() = c.to_rgba();
        self
    }
}

/// For objects that maintain a stack of colors for making drawing
/// commands easier.
pub trait StackedColored: Colored {
    /// Pushes a set (fill, stroke) of colors onto the stack as a clone
    /// of the previous colors.
    fn push_colors(&mut self);

    /// Pops a set (fill, stroke) of colors from the stack.
    fn pop_colors(&mut self);

    /// Executes a closure inside a pair of `push_colors()`, `pop_colors()`
    /// in order to provide a clean area for modifying the colors of the object
    /// without effecting things further down the line.
    ///
    /// Example:
    /// lux.with_colors(|lux| {
    ///   lux.fill_color(RED);
    ///   lux.stroke_color(BLUE);
    ///   // Draw some things.
    /// });
    fn with_colors<F>(&mut self, f: F) where F: FnOnce(&mut Self) {
        self.push_colors();
        f(self);
        self.pop_colors();
    }

    /// Same as `with_colors` but automatically sets the fill color for the
    /// duration of the closure.
    fn with_fill_color<C: Color, F>(&mut self, color: C, f: F)
    where F: FnOnce(&mut Self) {
        self.push_colors();
        self.fill_color(color);
        f(self);
        self.pop_colors();
    }

    /// Same as `with_colors` but automatically sets the stroke color for the
    /// duration of the closure.
    fn with_stroke_color<C: Color, F>(&mut self, color: C, f: F)
    where F: FnOnce(&mut Self) {
        self.push_colors();
        self.stroke_color(color);
        f(self);
        self.pop_colors();
    }
}
