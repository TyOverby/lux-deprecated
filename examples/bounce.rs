extern crate lux;

use lux::prelude::*;

fn bound(low: f32, value: f32, high: f32) -> (f32, bool) {
    if value <= low {
        (low , true)
    } else if value >= high {
        (high, true)
    } else {
        (value, false)
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
