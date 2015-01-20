extern crate lux;
use lux::*;

// TODO: when borders are better, add a border_rect example
fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        let mut frame = window.cleared_frame([0.9, 0.9, 0.9]);
        // draw_rect((x, y), (width, height))
        frame.draw(&rect((10.0, 20.0), (15.0, 25.0)));
        frame.with_fill_color([1.0, 0.0, 0.0], |frame| {
            frame.draw(&rect((20.0, 10.0), (10.0, 10.0)));
        });
    }
}
