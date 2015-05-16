extern crate lux;
use lux::prelude::*;
use lux::primitive_canvas::PrimitiveCanvas;

fn main() {
    let mut window = Window::new().unwrap();
    let vtxs = [
        ColorVertex {pos: [0.0, 0.0], color: colors::RED},
        ColorVertex {pos: [0.0, 200.0], color: colors::GREEN},
        ColorVertex {pos: [200.0, 0.0], color: colors::BLUE},
    ];
    let idxs = [0, 1, 2];

    while window.is_open() {
        let mut frame = window.cleared_frame(colors::WHITE);
        frame.draw_colored(TrianglesList, &vtxs[..], Some(&idxs[..]), None);
    }
}
