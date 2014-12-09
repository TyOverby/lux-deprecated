#![feature(if_let)]
extern crate lux;
use lux::{LuxCanvas, LuxWindow, Window};

fn main() {
    fn clamp(l: f32, v: f32, u: f32) -> Option<f32> {
        if v <= l {
            Some(l)
        } else if v >= u {
            Some(u)
        } else {
            None
        }
    }
    let mut lux = Window::new().unwrap();

    let sz = 10.0;
    let (mut x, mut y) = (3.0, 10.0);
    let (mut vx, mut vy) = (3.0, 3.0);
    while lux.is_open() {
        lux.clear([0.9, 0.9, 0.9, 0.1]);
        x += vx;
        y += vy;

        if let Some(nx) = clamp(0.0, x, lux.width() as f32 - sz) {
            x = nx;
            vx *= -1.0;
        }

        if let Some(ny) = clamp(0.0, y, lux.height() as f32 - sz) {
            y = ny;
            vy *= -1.0;
        }

        lux.draw_rect((x, y), (sz, sz));
        lux.render();
    }
}

