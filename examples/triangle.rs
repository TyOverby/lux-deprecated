extern crate lux;
extern crate vecmath;

use lux::prelude::*;
use lux::graphics::{PrimitiveCanvas, TrianglesList, ColorVertex};
use lux::color;

fn main() {
    let mut lux = Window::new().unwrap();

    while lux.is_open() {
        let mut frame = lux.cleared_frame(color::WHITE);
        *frame.current_matrix_mut() = vecmath::mat4_id();
        let vtxs = [
            ColorVertex {pos: [-0.5, -0.5], color: rgb(1.0, 0.0, 0.0)},
            ColorVertex {pos: [0.0, 0.5], color: rgb(0.0, 0.0, 1.0)},
            ColorVertex {pos: [0.5, -0.5], color: rgb(0.0, 1.0, 0.0)},
        ];

        frame.draw_colored(TrianglesList, &vtxs[..], None, None).unwrap();
    }
}
