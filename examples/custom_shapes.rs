extern crate lux;

use lux::prelude::*;
use lux::color;
use lux::graphics::{PrimitiveCanvas, TrianglesList, ColorVertex};

// Standard RGB triangle

fn main() {
    let mut window = Window::new_with_defaults().unwrap();
    let vtxs = [
        ColorVertex {pos: [0.0, 0.0], color: color::RED},
        ColorVertex {pos: [0.0, 200.0], color: color::GREEN},
        ColorVertex {pos: [200.0, 0.0], color: color::BLUE},
    ];
    let idxs = [0, 1, 2];

    while window.is_open() {
        let mut frame = window.cleared_frame(color::WHITE);
        frame.draw_colored(TrianglesList, &vtxs[..], Some(&idxs[..]), None).unwrap();
    }
}
