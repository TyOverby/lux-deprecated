extern crate lux;
use lux::{Interactive, Window, LuxCanvas};
use lux::keycodes::Escape;

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        // You can check keys by the character that they
        // produce, the special name, and the raw key-code.

        let color = if window.is_key_pressed(' ') {
            [0.5, 0.0, 0.0]
        } else if window.is_key_pressed(Escape) {
            [0.0, 0.0, 0.5]
        } else if window.is_key_pressed(38 /* 'a' */) {
            [0.0, 0.5, 0.0]
        } else {
            [0.8, 0.8, 0.8]
        };

        let mut frame = window.cleared_frame(color);
        frame.rect((0.0, 0.0), (20.0, 20.0)).fill();
    }
}
