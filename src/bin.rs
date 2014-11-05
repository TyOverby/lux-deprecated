extern crate lovely;

use lovely::window::Window;
use lovely::{LovelyCanvas, LovelyWindow, color, Vertex};

fn main() {
    let mut window = Window::new().unwrap();

    let shape = window.stamp_shape(&[
        Vertex { pos: [20.0, 20.0], tex: [0.0, 0.0] },
        Vertex { pos: [20.0, 40.0], tex: [0.0, 0.0] },
        Vertex { pos: [40.0, 40.0], tex: [0.0, 0.0] },
    ]);

    while window.is_open() {
        window.clear(color::consts::CYAN);
        window.draw_rect((0.0, 0.0), (0.0, 0.0));
        window.with_scale(10.0, 10.0, |window| {
            window.draw_shape(&shape);
        });
        window.render();
    }
}
