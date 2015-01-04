extern crate lux;
use lux::{LuxCanvas, StackedColored, Interactive, Window};

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        // Query the mouse position
        let (x, y) = window.mouse_pos();
        let down = window.mouse_down();

        let mut frame = window.frame([0.9, 0.9, 0.9]);
        // Query the state of the mouse buttons.  mouse_down() will return
        // true if *any* mouse buttons are down.
        let color = if down { [0.5, 0.0, 0.0] } else { [0.0, 0.0, 0.0] };
        frame.with_fill_color(color, |frame| {
            frame.rect((x as f32 - 50.0, y as f32 - 50.0), (100.0, 100.0)).fill();
        });
    }
}
