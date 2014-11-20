extern crate lux;

use lux::{LuxCanvas, LuxWindow, Vertex, TriangleList, Window};

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        window.clear([0.5, 0.5, 0.5]);
        window.draw_border_elipse((500.0, 500.0), (10.0, 30.0), 5.0);
        window.render();
    }
}
