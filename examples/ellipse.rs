extern crate lux;
use lux::*;

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        let mut frame = window.cleared_frame([0.9, 0.9, 0.9]);
        // draw_ellipse((x, y), (width, height))
        frame.draw(&ellipse((100.0, 20.0), (10.0, 20.0)));
        // draw_circle((x, y), radius)
        frame.draw(&circle((20.0, 20.0), 45.0));

        frame.draw(&circle((20.0, 100.0), 45.0));

        //window.draw_border_ellipse((20.0, 100.0), (50.0, 100.0), 5.0);
    }
}
