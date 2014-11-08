extern crate lovely;

use lovely::window::Window;
use lovely::{LovelyCanvas, LovelyWindow, Vertex};

fn main() {
    let mut window = Window::new().unwrap();

    let shape = window.stamp_shape(&[
        Vertex { pos: [20.0, 20.0], tex: [0.0, 0.0] },
        Vertex { pos: [20.0, 40.0], tex: [0.0, 0.0] },
        Vertex { pos: [40.0, 40.0], tex: [0.0, 0.0] },
    ]);

    while window.is_open() {
        window.process_events();
        window.clear([0.5, 0.5, 0.5]);
        let (x, y) = window.mouse_pos();
        window.with_color([1.0f32, 0.0, 1.0, 0.1], |window| {
            window.draw_elipse((x as f32, y as f32), (10.0, 20.0));
            window.draw_border_rect((50.0, 50.0), (40.0, 40.0), 5.0);
        });
        window.with_scale(10.0, 10.0, |window| {
            window.draw_shape(&shape);
        });

        let (start, end) = ((30.0, 10.0), (100.0, 100.0));
        let sz = (5.0, 5.0);
        window.draw_rect(start, sz);
        window.draw_rect(end, sz);
        window.with_color([0.0,0.0,1.0, 0.5], |window| {
            window.draw_line(start, end, 50.0);
        });
        window.render();
    }
}
