extern crate lux;

use lux::{
    LuxCanvas,
    PrimitiveCanvas,
    LuxWindow,
    Window,
    Colored,
    Vertex,
    TriangleList
};

use lux::colors;

fn main() {
    let mut lux = Window::new().unwrap();
    let mut delta = 0.0f32;

    while lux.is_open() {
        lux.clear([1.0, 0.0, 0.0, 0.1]);
        delta += 0.1;


        lux.with_rotation(delta, |lux|{
            for x in range(0u, 100) {
                for y in range(0u, 100) {
                    let (x, y) = (x as f32 * 40.0, y as f32 * 40.0);
                    lux.rect((x, y), (30.0, 30.0))
                       .fill_color(colors::BLUE)
                       .fill();
                }
            }

            let vtxs = [
                Vertex {pos: [0.0, 0.0], color: [1.0, 0.0, 0.0, 1.0], tex: [0.0, 0.0]},
                Vertex {pos: [0.0, 200.0], color: [0.0, 0.0, 1.0, 1.0], tex: [0.0, 0.0]},
                Vertex {pos: [200.0, 0.0], color: [0.0, 1.0, 0.0, 1.0], tex: [0.0, 0.0]},
            ];

            let idxs = [0, 1, 2];
            lux.draw_shape(TriangleList, vtxs.as_slice(), Some(idxs.as_slice()), None);
        });

        lux.render();
    }
}
