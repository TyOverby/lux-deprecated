extern crate lux;
use lux::{LuxCanvas, Interactive, Window};

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        window.clear([0.9, 0.9, 0.9]);
        // draw_ellipse((x, y), (width, height))
        window.ellipse((100.0, 20.0), (10.0, 20.0)).fill();
        // draw_circle((x, y), radius)
        window.circle((20.0, 20.0), 45.0).fill();

        //window.draw_border_ellipse((20.0, 100.0), (50.0, 100.0), 5.0);
        window.render();
    }
}
