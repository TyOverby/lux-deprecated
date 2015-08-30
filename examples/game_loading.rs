extern crate lux;

use lux::prelude::*;
use lux::game::*;

#[derive(Debug)]
struct MyGame {
    one: Option<u32>,
    two: Option<f64>,
    text: Option<String>
}

impl Game for MyGame {
    fn load(loader: &mut Loader<MyGame>) {
        loader.do_async("One",
        ||{
            std::thread::sleep_ms(100);
            Ok(1u32)
        }, |res, _window, game| {
            game.one = Some(res);
            Ok(())
        });

        loader.do_async("Two",
        ||{
            std::thread::sleep_ms(200);
            Ok(2.0f64)
        }, |res, _window, game| {
            game.two= Some(res);
            Ok(())
        });

        loader.do_async("Text",
        ||{
            std::thread::sleep_ms(100);
            Ok("Hello World".into())
        }, |res, _window, game| {
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
