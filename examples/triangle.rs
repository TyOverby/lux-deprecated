extern crate lux;
extern crate vecmath;

use lux::prelude::*;
use lux::primitive_canvas::PrimitiveCanvas;

fn main() {
    let mut lux = Window::new().unwrap();

    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::WHITE);
        *frame.current_matrix_mut() = vecmath::mat4_id();
        let vtxs = [
            ColorVertex {pos: [-0.5, -0.5], color: rgb(1.0, 0.0, 0.0)},
            ColorVertex {pos: [0.0, 0.5], color: rgb(0.0, 0.0, 1.0)},
            ColorVertex {pos: [0.5, -0.5], color: rgb(0.0, 1.0, 0.0)},
        ];

        frame.draw_shape(TrianglesList, &vtxs[..], None, None);
    }
}
