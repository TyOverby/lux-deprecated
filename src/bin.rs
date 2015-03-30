extern crate lux;
extern crate glium;
extern crate image;
extern crate freetype;

use lux::*;
use std::path::Path;
use std::fs::File;

fn main() {
    let mut lux = Window::new().unwrap();

    let mut f1 = File::create("out1.png").unwrap();
    let mut f2 = File::create("out2.png").unwrap();
    let png = ::image::ImageFormat::PNG;


    let freetype = freetype::Library::init().unwrap();

    let font1 = Path::new("./resources/SourceCodePro-Regular.ttf");
    let font2 = Path::new("./resources/cbt.ttf");

    let mut face1 = freetype.new_face(&font1, 0).unwrap();
    face1.set_pixel_sizes(0, 48).unwrap();

    let mut face2 = freetype.new_face(&font2, 0).unwrap();
    face2.set_pixel_sizes(0, 48).unwrap();

    let (s1, _) = gen_sheet(|img: image::DynamicImage | {
            let img = img.flipv();
            img.save(&mut f1, png).ok();
            let img = img.flipv();
            lux.sprite_from_image(img)
    }, &mut face1, 30).unwrap();

    {
        let mut frame = lux.cleared_frame(colors::BLACK);
        frame.rect(0.0, 0.0, 50.0, 50.0).fill();
    }

    let (s2, _) = gen_sheet(|img: image::DynamicImage | {
            let img = img.flipv();
            img.save(&mut f2, png).ok();
            let img = img.flipv();
            lux.sprite_from_image(img)
    }, &mut face2, 50).unwrap();

    println!("{:?}, {:?}", s1.sprite, s2.sprite);


    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::BLACK);
        frame.sprite(&s1.sprite, 0.0, 0.0).draw();
        frame.sprite(&s2.sprite, lux.mouse_x(), lux.mouse_y()).draw();
    }
}

/*
fn main() {
    let mut lux = Window::new().unwrap();
//    lux.load_font("SourceCodePro", &Path::new("./resources/SourceCodePro-Regular.ttf")).unwrap();

    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::WHITE);
        println!("\n\n next \n\n");
        frame.set_font("SourceCodePro", 10).unwrap();
        frame.draw_text("foo", lux.mouse_x(), lux.mouse_y()).unwrap();
    }
}
*/

