extern crate lux;

use lux::{LuxCanvas, LuxWindow, Window, StackedTransform, Colored, Transform};
use lux::colors;

fn main() {
    let mut lux = Window::new().unwrap();
    let mut theta:f32 = 0.0;
    while lux.is_open() {
        lux.clear([0.9, 0.9, 0.9]);
        theta += 0.01;

        lux.rect((10.0, 20.0), (10.0, 10.0))
            .translate(10.0, 10.0)
            .fill_color([0.0, 0.0, 1.0, 1.0])
            .fill();

        lux.rect((20.0, 10.0), (10.0, 10.0))
            .fill_color([1.0, 0.0, 0.0, 0.5])
            .fill();

        lux.render();
    }
}
