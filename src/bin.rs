extern crate lovely;

use lovely::window::Window;
use lovely::{Lovely, color, Vertex};

fn main() {
    let mut window = Window::new().unwrap();
    let shape = window.stamp_shape(&[
        Vertex { pos: [20.0, 20.0], tex: [0.0, 0.0] },
        Vertex { pos: [20.0, 40.0], tex: [0.0, 0.0] },
        Vertex { pos: [40.0, 40.0], tex: [0.0, 0.0] },
    ]);
    loop {
        window.clear(color::consts::CYAN);
        window.draw_rect((0.0, 0.0), (0.0, 0.0));
        window.push_matrix();
        window.scale(5.0, 5.0);
        window.draw_shape(&shape);
        window.pop_matrix();
        window.render();
    }
}
