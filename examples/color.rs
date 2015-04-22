extern crate lux;
use lux::prelude::*;

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        let mut frame = window.cleared_frame(rgb(0.9, 0.9, 0.9));
        frame.with_color([1.0, 0.0, 0.0, 0.5], |frame| {
            frame.rect(0.0, 0.0, 100.0, 100.0).fill();
            frame.rect(50.0, 50.0, 100.0, 100.0).fill();
        });
    }
}
