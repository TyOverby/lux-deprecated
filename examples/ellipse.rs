extern crate lux;

use lux::prelude::*;

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        let mut frame = window.cleared_frame([0.9, 0.9, 0.9]);
        frame.color(rgba(0.0, 0.0, 0.0, 0.8));

        frame.circle(20.0, 20.0, 45.0)
             .fill();

        // Manually set the number of segments to build the ellipse out of.
        frame.ellipse(100.0, 20.0, 100.0, 50.0)
             .segments(8)
             .fill();

        // Manually set the length of each line segment in the ellipse.
        frame.circle(20.0, 100.0, 200.0)
             .line_length(15)
             .fill();
    }
}
