extern crate lux;
use lux::prelude::*;

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        // clear([r, g, b]) OR
        // clear([r, g, b, a])
        let mut frame = window.cleared_frame(rgb(0.9, 0.9, 0.9));
        // with_color([r, g, b], closure)
        // with_color([r, g, b, a], closure)
        frame.with_fill_color([1.0, 0.0, 0.0, 0.5], |frame| {
            frame.rect(0.0, 0.0, 100.0, 100.0).fill();
            frame.rect(50.0, 50.0, 100.0, 100.0).fill();
        });
    }
}
