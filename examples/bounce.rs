extern crate lux;
use lux::prelude::*;

fn bound(l: f32, v: f32, h: f32) -> (f32, bool) {
    if v <= l{
        (l, true)
    } else if v >= h {
        (h, true)
    } else {
        (v, false)
    }
}

fn main() {
    let mut window = Window::new().unwrap();
    let size = 10.0f32;
    let (mut x, mut y) = (20.0, 50.0);
    let (mut vx, mut vy) = (1.5, 1.5);
    while window.is_open() {
        let mut frame = window.cleared_frame([0.9, 0.9, 0.9, 0.001]);
        x += vx;
        y += vy;

        let (nx, cx) = bound(0.0, x, frame.width() - size);
        if cx {
            x = nx;
            vx = - vx;
        }

        let (ny, cy) = bound(0.0, y, frame.height() - size);
        if cy {
            y = ny;
            vy = - vy;
        }

        frame.square(x, y, size).fill();
    }
}
