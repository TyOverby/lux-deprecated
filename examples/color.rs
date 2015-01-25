extern crate lux;
use lux::*;

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        // clear([r, g, b]) OR
        // clear([r, g, b, a])
        let mut frame = window.cleared_frame(rgb(0.9, 0.9, 0.9));
        // with_color([r, g, b], closure)
        // with_color([r, g, b, a], closure)
        frame.with_fill_color(rgba(1.0, 0.0, 0.0, 0.5), |frame| {
            frame.draw(&rect((0.0,0.0), (100.0, 100.0)));
            frame.draw(&rect((50.0,50.0), (100.0, 100.0)));
        });
    }
}
