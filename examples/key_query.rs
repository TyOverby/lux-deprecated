extern crate lux;

use lux::prelude::*;
use lux::color;
use lux::interactive::keycodes::Escape;

const MESSAGE: &'static str =
    "Press [space] [esc] or [a] to change the background color";

fn main() {
    let mut window = Window::new_with_defaults().unwrap();
    while window.is_open() {

        // You can check keys by the character that they
        // produce, the special name, and the raw key-code.
        let (bg_color, text_color) = if window.is_key_pressed(' ') {
            (rgb(0.5, 0.0, 0.0), color::BLACK)
        } else if window.is_key_pressed(Escape) {
            (rgb(0.0, 0.0, 0.5), color::WHITE)
        } else if window.is_key_pressed(38) {
            // 38 is the char code for `a`.
            (rgb(0.0, 0.5, 0.0), color::BLACK)
        } else {
            (rgb(0.5, 0.5, 0.5), color::BLACK)
        };

        let mut frame = window.cleared_frame(bg_color);
        frame.text(MESSAGE, 0.0, 50.0).color(text_color).draw().unwrap();
    }
}
