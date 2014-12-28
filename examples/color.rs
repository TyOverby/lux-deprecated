extern crate lux;
use lux::{LuxCanvas, StackedColored, Interactive, Window};

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        // clear([r, g, b]) OR
        // clear([r, g, b, a])
        window.clear([0.9, 0.9, 0.9]);
        // with_color([r, g, b], closure)
        // with_color([r, g, b, a], closure)
        window.with_fill_color([1.0, 0.0, 0.0, 0.5], |window| {
            window.rect((0.0,0.0), (100.0, 100.0)).fill();
            window.rect((50.0,50.0), (100.0, 100.0)).fill();
        });
        window.render();
    }
}