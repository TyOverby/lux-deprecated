extern crate lux;
use lux::*;

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        let mut frame = window.cleared_frame([0.9, 0.9, 0.9]);
        // draw_line(start, end, line_width)
        frame.draw_line((5.0, 10.0), (30.0, 60.0), 5.0);
        // draw_lines(points, line_width)
        frame.draw_lines([
            (50.0, 50.0), (150.0, 50.0), (150.0, 150.0), (50.0, 150.0)
        ].iter().map(|a| *a), 10.0);
    }
}
