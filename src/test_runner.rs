#![allow(dead_code)]

extern crate lux;
use std::collections::VecDeque;
use lux::prelude::*;
use lux::graphics::ColorVertex;
use lux::interactive::keycodes::Escape;
use lux::interactive::Event;

struct TestRunner {
    tests: VecDeque<(String, Box<FnMut(&mut Frame)>)>
}

impl TestRunner {
    fn new() -> TestRunner {
        TestRunner{ tests: VecDeque::new() }
    }

    fn add_test<F: FnMut(&mut Frame) + 'static>(&mut self, name: &str, f: F) {
        self.tests.push_back((name.to_string(), Box::new(f) as Box<FnMut(&mut Frame)>));
    }

    fn display(&mut self) -> LuxResult<()> {
        let mut window = try!(Window::new());

        let mut current = self.tests.pop_front();
        while window.is_open() && current.is_some() {
            {
                let cur = current.as_mut().unwrap();
                let mut frame = window.cleared_frame(rgb(255, 255, 255));

                frame.text(&cur.0, 0.0, 0.0).draw().unwrap();
                frame.with_translate(0.0, 50.0, |frame| cur.1(frame));
            }

            let mut close = false;
            if window.events().any(|e| match e {
                Event::KeyPressed(_, Some(' '), _) => true,
                Event::KeyPressed(_, _, Some(Escape)) => {
                    close = true;
                    false
                },
                _ => false
            }){
                current = self.tests.pop_front();
            }

            if close {
                break;
            }
        }

        Ok(())
    }
}

const PI: f32 = 3.14159;
const PI_4: f32 = PI / 4.0;

fn main() {
    let mut runner = TestRunner::new();


    runner.add_test("text_with_newline", |frame| {
        frame.text("Hello\nWorld", 0.0, 0.0).draw().unwrap();
    });

    runner.add_test("indiv_rotated_squares", |frame| {
        frame.color(rgb(255, 100, 0));

        for i in (0 .. 5) {
            let border = i as f32 * 10.0;
            let pos = i as f32 * 100.0;
            frame.square(pos, 0.0, 50.0)
                 .border(border / 2.0, rgb(255, 100, 50))
                 .rotate_around((12.5, 12.5), PI_4 + 0.2)
                 .fill_and_stroke();
        }
    });

    runner.add_test("squares", |frame| {
        frame.color(rgb(255, 0, 0));

        for i in (0 .. 5) {
            let border = i as f32 * 10.0;
            let pos = i as f32 * 100.0;
            frame.square(pos, 0.0, 50.0)
                 .border(border / 2.0, rgba(0, 0, 255, 100))
                 .fill_and_stroke();
        }
    });

    runner.add_test("rotated_squares", |frame| {
        frame.rotate(0.5);
        frame.color(rgb(255, 0, 0));

        for i in (0 .. 5) {
            let border = i as f32 * 10.0;
            let pos = i as f32 * 100.0;
            frame.square(pos, 0.0, 50.0)
                 .border(border / 2.0, rgba(0, 0, 255, 100))
                 .fill_and_stroke();
        }
    });

    runner.add_test("red_square_rotated_frame", |frame| {
        frame.color(rgb(255, 0, 0));
        frame.with_rotate_around((12.5, 12.5), 0.5, |frame| {
            frame.square(0.0, 0.0, 25.0).fill();
        });
    });

    runner.add_test("red_square_rotated_self", |frame| {
        frame.color(rgb(255, 0, 0));
        frame.square(0.0, 0.0, 25.0).rotate_around((12.5, 12.5), 0.5).fill();
        frame.color(rgb(0, 0, 255));
        frame.square(50.0, 50.0, 25.0).rotate_around((12.5, 12.5), 0.5).fill();
    });

    runner.add_test("alpha_blending", |frame| {
        frame.color(rgba(1.0, 0.0, 0.0, 1.0));
        frame.square(0.0, 0.0, 25.0).fill();

        frame.rotate(0.5);
        frame.color(rgba(0.0, 0.0, 1.0, 0.5));
        frame.square(12.0, 12.0, 25.0).fill();
    });

    runner.add_test("font_stuff", |frame| {
        frame.text("abcdefg", 0.0, 25.0).draw().unwrap();

        frame.text("hijklmnop", 0.0, 25.0)
             .size(30)
             .color(rgba(1.0, 0.0, 0.0, 1.0))
             .draw().unwrap();

        frame.text("hijklmnop", 0.0, 25.0)
             .size(10)
             .color(rgba(1.0, 0.0, 0.0, 0.5))
             .draw().unwrap();
    });

    runner.add_test("sprite_sheet", |frame| {
        let sp = frame.load_texture_file("test/test.png").unwrap().to_sprite();
        let mc = frame.load_texture_file("test/minecraft_fixedwidth_font.png").unwrap().to_sprite();
        let mc = mc.sub_sprite((0, 0), (200, 200)).unwrap();
        frame.sprite(&sp, 0.0, 0.0).draw();
        frame.sprite(&mc, 50.0, 50.0).size(100.0, 100.0).draw();
    });

    runner.add_test("points", |frame| {
        let mut v = Vec::with_capacity(256 * 256);
        for x in 0 .. 255 {
            for y in 0 .. 255 {
                let vl = x ^ y;
                v.push(ColorVertex {
                    pos: [x as f32, y as f32],
                    color: rgb(vl, vl, vl)
                });
            }
        }

        frame.draw_points(&v);
    });

    runner.add_test("point", |frame| {
        for y in 0 .. 50 {
            let y = y as f32;
            frame.draw_point(0.5, y + 0.5, rgb(255, 0, 0));
            frame.draw_point(y + 0.5, 0.5, rgb(0, 255, 0));
            frame.draw_point(y + 1.5, y + 0.5, rgb(0, 0, 255));
        }
    });

    runner.display().unwrap();
}
