use vecmath::{mat4_id, col_mat4_mul};

pub trait LuxRaw {
    fn current_matrix(&self) -> &[[f32, ..4], ..4];
    fn current_matrix_mut(&mut self) -> &mut [[f32, ..4], ..4];
    fn push_matrix(&mut self);
    fn pop_matrix(&mut self);
    fn with_matrix(&mut self, f: |&Self|) {
        self.push_matrix();
        f(self);
        self.pop_matrix();
    }
    fn apply_matrix(&mut self, matrix: [[f32, ..4], ..4]) {
        let current = self.current_matrix_mut();
        *current = col_mat4_mul(*current, matrix);
    }
    fn translate(&mut self, dx: f32, dy: f32) {
        let mut prod = mat4_id();
        prod[3][0] = dx;
        prod[3][1] = dy;
        self.apply_matrix(prod);
    }
    fn scale(&mut self, sx: f32, sy: f32) {
        let mut prod = mat4_id();
        prod[0][0] = sx;
        prod[1][1] = sy;
        self.apply_matrix(prod);
    }
    fn shear(&mut self, sx: f32, sy: f32) {
        let mut prod = mat4_id();
        prod[1][0] = sx;
        prod[0][1] = sy;
        self.apply_matrix(prod);
    }
    fn rotate(&mut self, theta: f32) {
        let mut prod = mat4_id();
        let (c, s) = (theta.cos(), theta.sin());
        prod[0][0] = c;
        prod[0][1] = s;
        prod[1][0] = -s;
        prod[1][1] = c;
        self.apply_matrix(prod);
    }
}
