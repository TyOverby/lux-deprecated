extern crate lux;

use lux::prelude::*;
use lux::game::*;

#[derive(Debug)]
struct MyGame {
    one: Option<u32>,
    two: Option<f64>,
    text: Option<String>
}

fn long_random_time() -> u32 {
    // 3 seconds.
    3000
}

fn small_random_time() -> u32 {
    1000
}

impl Game for MyGame {
    fn load(loader: &mut Loader<MyGame>) {
        for _ in 0 .. 5 {
            loader.do_async("load the number 1",
            ||{
                std::thread::sleep_ms(long_random_time());
                Ok(1u32)
            }, |res, _window, game| {
                std::thread::sleep_ms(small_random_time());
                game.one = Some(res);
                Ok(())
            });
        }

        loader.do_async("load the number 2.0",
        ||{
            std::thread::sleep_ms(long_random_time());
            Ok(2.0f64)
        }, |res, _window, game| {
            std::thread::sleep_ms(small_random_time());
            game.two= Some(res);
            Ok(())
        });

        loader.do_async("load an intro message",
        ||{
            std::thread::sleep_ms(long_random_time());
            Ok("Hello World".into())
        }, |res, _window, game| {
            std::thread::sleep_ms(small_random_time());
            game.text = Some(res);
            Ok(())
        });
    }

    fn update(&mut self, _: f32, _: &mut Window, _: &mut EventIterator) -> LuxResult<()> {
        println!("{:?}", self);
        std::process::exit(0);
    }

    fn render(&mut self, _: f32, _: &mut Window, _: &mut Frame) -> LuxResult<()> {
        Ok(())
    }
}

fn main() {
    let game = MyGame {
        one: None,
        two: None,
        text: None
    };

    game.run_until_end().unwrap();
}
