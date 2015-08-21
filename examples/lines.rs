extern crate lux;

use lux::prelude::*;

fn main() {
    let mut window = Window::new().unwrap();
    let mut a = 0.5;
    let mut b = 0.0;
    while window.is_open() {
        let mut frame = window.cleared_frame(rgb(0.9, 0.9, 0.9));

        frame.draw_line(50.0, 50.0, 100.0, 100.0, 15.0);
        frame.draw_lines(vec![(100.0, 100.0),window.mouse_pos(),(0.0, 100.0)].into_iter(), 10.0);
        frame.draw_arc((200.0, 200.0), 100.0, a, b, 4.0);

        a += 0.001;
        b += 0.001;
    }
}
