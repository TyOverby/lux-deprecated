extern crate lux;
extern crate num;

use lux::prelude::*;
use lux::color;
use lux::graphics::StencilType;

use num::Float;

const SIZE:f32 = 10.0;
const DIST_2:f32 = (2.0 * SIZE * SIZE);

fn draw_field(
    frame: &mut Frame,
    stenc: StencilType,
    (x, y, theta): (f32, f32, f32),
    color: [f32; 4]) {

    frame.clear_stencil(stenc);
    frame.draw_to_stencil(stenc.inverse(), |frame| {
        frame.draw(Circle {
            x: x,
            y: y,
            diameter: 600.0,
            color: color::BLACK,
            .. Default::default()
        }).unwrap();
    });

    for x in 0 .. 43 {
        for y in 0 .. 43 {
            let x = x as f32 * DIST_2.sqrt();
            let y = y as f32 * DIST_2.sqrt();
            frame.draw(Square {
                x: x, y: y,
                size: SIZE,
                color: color,
                transform: Some(*mat4_id().rotate_around((x + SIZE / 2.0, y + SIZE / 2.0), theta))
            });
        }
    }
}

fn main() {
    let mut window = Window::new_with_defaults().unwrap();

    let mut theta: f32 = 0.0;
    let (x, y) = (0.0, 0.0);
    while window.is_open() {
        let mut frame = window.cleared_frame(rgb(0.9, 0.9, 0.9));
        theta += 0.01;
        draw_field(&mut frame, StencilType::Allow, (x, y, theta), color::RED);
        draw_field(&mut frame, StencilType::Deny, (x, y, theta), color::BLUE);
    }
}
