extern crate lux;

use lux::{LuxCanvas, LuxWindow, Window, Transform};

fn main() {
    let mut lux = Window::new().unwrap();
    while lux.is_open() {
        lux.clear([0.9, 0.9, 0.9]);

        for x in range(0u, 200) {
            for y in range(0u, 200) {
                lux.rect((x as f32 * 10.0, y as f32 * 10.0), (10.0, 10.0))
                   .padding(1.0)
                   .rotate(2.0 * 3.14 / 8.0)
                   .fill();
            }
        }

        let pos = (lux.mouse_x() as f32, lux.mouse_y() as f32);
        lux.rect(pos, (20.0, 20.0)).rotate_around((10.0, 10.0), 0.2).fill();

        lux.render();
    }
}

