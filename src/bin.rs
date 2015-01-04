extern crate lux;

use lux::{
    LuxCanvas,
    PrimitiveCanvas,
    StackedTransform,
    Interactive,
    Window,
    Colored,
    Vertex,
    TrianglesList
};

use lux::colors;

fn main() {
    let mut lux = Window::new().unwrap();
    let mut delta = 0.0f32;

    while lux.is_open() {
        delta += 0.1;

        let mut frame = lux.frame(colors::RED);

        frame.with_rotation(delta, |frame|{
          for x in range(0u, 100) {
                for y in range(0u, 100) {
                    let (x, y) = (x as f32 * 40.0, y as f32 * 40.0);
                    frame.rect((x, y), (30.0, 30.0))
                       .fill_color(colors::BLUE)
                       .fill();
                }
            }

            let vtxs = [
                Vertex {pos: [0.0, 0.0], color: [1.0, 0.0, 0.0, 1.0]},
                Vertex {pos: [0.0, 200.0], color: [0.0, 0.0, 1.0, 1.0]},
                Vertex {pos: [200.0, 0.0], color: [0.0, 1.0, 0.0, 1.0]},
            ];

            let idxs = [0, 1, 2];
            frame.draw_shape(TrianglesList, vtxs.as_slice(), Some(idxs.as_slice()), None);
        });

        frame.rect((100.0, 100.0), (50.0, 50.0)).fill_color(colors::GREEN).fill();

        frame.draw_pixel((100.5, 100.5), colors::RED);

        frame.draw_pixels(range(100i32, 200).map(|i| ((i as f32, i as f32), colors::BLUE)));

        frame.rect((100.0, 100.0), (50.0, 50.0)).fill_color(colors::GREEN).fill();

    }
}
