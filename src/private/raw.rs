use vecmath::{mat4_id, col_mat4_mul};
use super::types::Float;
use super::color::Color;

/// A trait for objects that can be "transformed".  Transformations
/// include scaling, translation, shearing, rotating, and general
/// purpose matrix application.
pub trait Transform {
    /// Return a reference to the current matrix.
    fn current_matrix(&self) -> &[[Float; 4]; 4];
    /// Return a mutible reference to the current matrix.
    fn current_matrix_mut(&mut self) -> &mut [[Float; 4]; 4];

    /// Multiplies the current matrix against another.
    /// `self = self * other`.
    fn apply_matrix(&mut self, other: [[Float; 4]; 4]) -> &mut Self{
        {
            let current = self.current_matrix_mut();
            *current = col_mat4_mul(*current, other);
        }
        self
    }

    /// Applies a translation transformation to the matrix.
    fn translate(&mut self, dx: Float, dy: Float) -> &mut Self {
        let mut prod = mat4_id();
        prod[3][0] = dx;
        prod[3][1] = dy;
        self.apply_matrix(prod)
    }

    /// Applies a scaling transformation to the matrix.
    fn scale(&mut self, sx: Float, sy: Float) -> &mut Self {
        let mut prod = mat4_id();
        prod[0][0] = sx;
        prod[1][1] = sy;
        self.apply_matrix(prod)
    }

    /// Applies a shearing transformation to the matrix.
    fn shear(&mut self, sx: Float, sy: Float) -> &mut Self {
        let mut prod = mat4_id();
        prod[1][0] = sx;
        prod[0][1] = sy;
        self.apply_matrix(prod)
    }

    /// Applies a rotation transformation to the matrix.
    fn rotate(&mut self, theta: Float) -> &mut Self {
        use num::traits::Float;
        let mut prod = mat4_id();
        let (c, s) = (theta.cos(), theta.sin());
        prod[0][0] = c;
        prod[0][1] = s;
        prod[1][0] = -s;
        prod[1][1] = c;
        self.apply_matrix(prod)
    }

    /// Combines rotation with translation to effectively
    /// rotate around a given point.
    fn rotate_around(&mut self, point: (Float, Float), theta: Float) -> &mut Self {
        self.translate(point.0, point.1);
        self.rotate(theta);
        self.translate(-point.0, -point.1);
        self
    }

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
    fn with_matrix<F, R>(&mut self, f: F) -> R
    where F: FnOnce(&mut Self) -> R{
        let prev = *self.current_matrix();
        let r = f(self);
        *self.current_matrix_mut() = prev;
        r
    }

    /// Similar to `with_matrix` but with a rotation applied
    /// for the duration of the closure.
    fn with_rotation<F, R>(&mut self, rotation: Float, f: F) -> R
    where F: FnOnce(&mut Self) -> R {
        let prev = *self.current_matrix();
        self.rotate(rotation);
        let r =  f(self);
        *self.current_matrix_mut() = prev;
        r
    }

    /// Similar to `with_matrix` but with a translation applied
    /// for the duration of the closure.
    fn with_translate<F, R>(&mut self, dx: Float, dy: Float, f: F) -> R
    where F: FnOnce(&mut Self) -> R {
        let prev = *self.current_matrix();
        self.translate(dx, dy);
        let r = f(self);
        *self.current_matrix_mut() = prev;
        r
    }

    /// Similar to `with_matrix` but with a scale applied
    /// for the duration of the closure.
    fn with_scale<F, R>(&mut self, scale_x: Float, scale_y: Float, f: F) -> R
    where F: FnOnce(&mut Self) -> R {
        let prev = *self.current_matrix();
        self.scale(scale_x, scale_y);
        let r = f(self);
        *self.current_matrix_mut() = prev;
        r
    }

    /// Similar to `with_matrix` but with a shear applied
    /// for the duration of the closure.
    fn with_shear<F, R>(&mut self, sx: Float, sy: Float, f: F) -> R
    where F: FnOnce(&mut Self) -> R {
        let prev = *self.current_matrix();
        self.shear(sx, sy);
        let r = f(self);
        *self.current_matrix_mut() = prev;
        r
    }

    /// Similar to `with_matrix` but with rotate_around applied
    /// for the duration of the closure.
    fn with_rotate_around<F, R>(&mut self, point: (Float, Float), theta: Float, f: F) -> R
    where F: FnOnce(&mut Self) -> R {
        let prev = *self.current_matrix();
        self.rotate_around(point, theta);
        let r = f(self);
        *self.current_matrix_mut() = prev;
        r
    }
}

/// Turns out that implementing matrix transformations on matrices is a
/// no brainer!
impl Transform for [[Float; 4]; 4] {
    fn current_matrix(&self) -> &[[Float; 4]; 4] { self }
    fn current_matrix_mut(&mut self) -> &mut [[Float; 4]; 4] { self }
}

/// A trait representing objects that can be colored with
/// seperate fill colors and stroke colors.
///
/// The basic representation of a color is an array of 4 floats
/// where each value goes from 0.0 to 1.0 and is of the form
/// [r, b, b, a].
pub trait Colored {
    /// Returns the current color.
    fn get_color(&self) -> [Float; 4];

    /// Sets the color.
    fn color<C: Color>(&mut self, color: C) -> &mut Self;

    /// Executes a closure with the given color, then resets it to what it was before.
    ///
    /// ### Example
    /// ```ignore,rust
    /// frame.with_color(rgb(255, 0, 0), |frame| {
    ///     frame.rect(0.0, 0.0, 50.0, 50.0).fill();
    ///     frame.rect(0.0, 51.0, 50.0, 50.0).fill();
    ///     frame.rect(51.0, 0.0, 50.0, 50.0).fill();
    /// });
    /// ```
    fn with_color<F, R, C: Color>(&mut self, color: C, f: F)
    where F: FnOnce(&mut Self) -> R {
        let prev = self.get_color();
        self.color(color);
        f(self);
        self.color(prev);
    }
}
