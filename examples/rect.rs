extern crate lux;
use lux::{LuxCanvas, StackedColored, Interactive, Window};

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        window.clear([0.9, 0.9, 0.9]);
        // draw_rect((x, y), (width, height))
        window.rect((10.0, 20.0), (15.0, 25.0)).fill();
        window.with_fill_color([1.0, 0.0, 0.0], |window| {
            window.rect((20.0, 10.0), (10.0, 10.0)).fill();
            // TODO: when borders are better, add a border_rect example
        });
        window.render();
    }
}
