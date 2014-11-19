extern crate lux;
use lux::{LuxCanvas, LuxWindow, Window};

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        window.clear([0.9, 0.9, 0.9]);
        // draw_line(start, end, line_width)
        window.draw_line((5.0, 10.0), (30.0, 60.0), 5.0);
        // draw_lines(points, line_width)
        window.draw_lines(&[
            (50.0, 50.0), (150.0, 50.0), (150.0, 150.0), (50.0, 150.0)
        ], 10.0);
        window.render();
    }
}
