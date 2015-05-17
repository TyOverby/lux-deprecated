extern crate lux;

use lux::prelude::*;

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        let mut frame = window.cleared_frame([0.9, 0.9, 0.9]);

        frame.ellipse(100.0, 20.0, 10.0, 20.0).fill();
        frame.circle(20.0, 20.0, 45.0).fill();
        frame.circle(20.0, 100.0, 45.0).fill();
    }
}
