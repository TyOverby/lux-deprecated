extern crate lux;
use lux::{LuxCanvas, LuxWindow, Window};

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        window.clear([0.9, 0.9, 0.9]);
        window.draw_rect((10.0, 20.0), (15.0, 25.0)); // (pos, size)
        window.with_color([1.0, 0.0, 0.0], |window| {
            window.draw_rect((20.0, 10.0), (10.0, 10.0));
            // TODO: when borders are better, add that
        });
        window.render();
    }
}