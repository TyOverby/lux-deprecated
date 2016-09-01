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

        frame.draw(Circle {x: 50.0, y: 50.0, size: 300.0, color: color::CADETBLUE, .. Default::default()}).unwrap();

        frame.draw(Square {x: 50.0, y: 50.0, size: 150.0, color: color::ROYALBLUE, .. Default::default()}).unwrap();
    }
}
