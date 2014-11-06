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

    let (mut x, mut y) = (0.0f32, 10.0f32);
    while window.is_open() {
        window.clear(color::consts::CYAN);
        window.draw_rect((x, y), (10.0f32, 10.0f32));
        x += 1.0;
        y += 1.0;
        window.with_color([1.0f32, 0.0, 1.0, 0.5], |window| {
            window.draw_border_rect((50.0, 50.0), (40.0, 40.0), 5.0);
        });
        window.with_scale(10.0, 10.0, |window| {
            window.draw_shape(&shape);
        });
        window.render();
    }
}
