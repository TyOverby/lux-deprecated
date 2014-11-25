extern crate lux;

use lux::{LuxCanvas, LuxWindow, Vertex, TriangleList, Window};

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        window.clear([0.5, 0.5, 0.5]);
        window.with_color([0.0, 0.0, 1.0], |window| {
            window.draw_rect((100.0, 100.0), (100.0, 300.0));
        });
        window.with_color([0.0, 1.0, 0.0], |window| {
            window.draw_border_elipse((100.0, 100.0), (100.0, 300.0), 40.0);
        });
        window.with_color([1.0, 1.0, 0.0, 0.25], |window| {
            window.draw_rect((100.0, 100.0), (200.0, 40.0));
        });
        window.with_color([0.0, 0.0, 1.0, 0.5], |window| {
            window.draw_rect((100.0, 100.0), (100.0, 300.0));
        });
        window.render();
    }
}
