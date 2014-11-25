#![feature(tuple_indexing)]
extern crate lux;

use lux::{LuxCanvas, LuxWindow, Vertex, TriangleList, Window};

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        window.clear([0.5, 0.5, 0.5]);
        let pos = (100.0, 100.0);
        let size = window.mouse_pos();
        let size = (size.0 as f32, size.1 as f32);
        window.with_color([0.0, 0.0, 1.0], |window| {
            window.draw_rect(pos, size);
        });
        window.with_color([0.0, 1.0, 0.0], |window| {
            window.draw_border_elipse(pos, size, 40.0);
        });
        window.with_color([1.0, 1.0, 0.0, 0.25], |window| {
            window.draw_rect(pos, size);
        });
        window.with_color([0.0, 0.0, 1.0, 0.5], |window| {
            window.draw_rect(pos, size);
        });
        window.render();
    }
}
