extern crate lux;

use lux::prelude::*;
use lux::color;
use lux::graphics::{PrimitiveCanvas, TrianglesList, ColorVertex};

fn main() {
    let mut window = Window::new_with_defaults().unwrap();

    let vtxs = [
        ColorVertex {pos: [0.0, 0.0],   color: color::RED},
        ColorVertex {pos: [0.0, 200.0], color: color::GREEN},
        ColorVertex {pos: [200.0, 0.0], color: color::BLUE},
    ];

    while window.is_open() {
        let mut frame = window.cleared_frame(color::WHITE);


        frame.draw_colored(TrianglesList, &vtxs[..], None, None).unwrap();

        frame.circle(50.0, 50.0, 300.0)
             .color(color::CADETBLUE)
             .fill();

        frame.rect(50.0, 50.0, 150.0, 150.0) // (x, y, w, h)
             .color(color::BLUE)
             .border(10.0, color::CADETBLUE)
             .fill_and_stroke();
    }
}
