extern crate lux;
use lux::prelude::*;
use lux::colors::{BLUE, RED, GRAY};

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        // Query the mouse position
        let (x, y) = window.mouse_pos();

        let mut frame = window.cleared_frame(GRAY);
        // Query the state of the mouse buttons.
        // mouse_down() will return true if *any* mouse buttons are down.
        let color = if window.mouse_down() { BLUE } else { RED };
        frame.with_fill_color(color, |frame| {
            frame.rect(x - 50.0, y - 50.0, 100.0, 100.0).fill();
        });

        frame.draw_text("Click the mouse to change the color!", 0.5, 50.5).unwrap();
    }
}
