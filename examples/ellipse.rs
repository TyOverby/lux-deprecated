extern crate lux;
use lux::{LuxCanvas, LuxWindow, Window};

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        window.clear([0.9, 0.9, 0.9]);
        // draw_ellipse((x, y), (width, height))
        window.draw_ellipse((100.0, 20.0), (10.0, 20.0));
        // draw_circle((x, y), radius)
        window.draw_circle((20.0, 20.0), 45.0);

        window.draw_border_ellipse((20.0, 100.0), (50.0, 100.0), 5.0);
        window.render();
    }
}
