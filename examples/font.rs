extern crate font_atlas;
extern crate lux;
extern crate image;

use lux::prelude::*;
use lux::color;
use std::borrow::Cow;
use std::path::Path;
use image::GenericImage;

fn main() {
    let mut lux = Window::new_with_defaults().unwrap();
    let mut cache = font_atlas::cache::FontCache::new();
    cache.load_font("Pacifico", font_atlas::load_font("./resources/Pacifico.ttf").unwrap());
    cache.load_font("cbt", font_atlas::load_font("./resources/cbt.ttf").unwrap());

    {
        let f = |bitmap: font_atlas::rasterize::Bitmap| -> LuxResult<Sprite> {
            let mut image = image::DynamicImage::new_rgba8(bitmap.width() as u32, bitmap.height() as u32);
            for (y, line) in bitmap.lines().enumerate() {
                for (x, &pixel) in line.iter().enumerate() {
                    image.put_pixel(x as u32, (bitmap.height() - 1 - y) as u32, image::Rgba{ data: [pixel, pixel, pixel, pixel] })
                }
            }
            lux.texture_from_image(image).map(Texture::into_sprite)
        };

        for size in 1 .. 6 {
            cache.create_face("Pacifico", (size * 10) as f32, font_atlas::ASCII.iter().cloned(), |a| f(a));
        }
    }

    while lux.is_open() {
        let mut frame = lux.cleared_frame(color::RED);
        let mut sprite = None;
        frame.rect(50.0, 50.0, 500.0, 500.0).fill();
        for dc in cache.drawing_commands("Pacifico", 50.0, "hello world").unwrap() {
            sprite = Some(dc.bitmap);
            let sub_sprite = dc.bitmap.sub_sprite((dc.bitmap_location.x, dc.bitmap_location.y),
                                                  (dc.bitmap_location.w, dc.bitmap_location.h)).unwrap();
            frame.sprite(&sub_sprite, dc.draw_location.0 + 50.0, dc.draw_location.1 + 50.0).draw();
        }
        frame.sprite(sprite.unwrap(), lux.mouse_pos().0, lux.mouse_pos().1).draw();

        /*
        let mut frame = lux.cleared_frame(color::WHITE);
        let mut y = 0.0;

        for pt in 1 .. 6 {
            frame.text("Hello World", 0.0, y)
                  .size(pt * 10)
                  .font("Pacifico")
                  .draw().unwrap();
            y += (pt * 15) as f32;
        }

        for pt in 1 .. 6 {
            frame.text("Hello World", 0.0, y)
                  .size(pt * 10)
                  .font("cbt")
                  .draw().unwrap();
            y += (pt * 15) as f32;
        }

        for pt in 1 .. 6 {
            frame.text("Hello World", 0.0, y)
                  .size(pt * 10)
                  .font("SourceCodePro")
                  .draw().unwrap();
            y += (pt * 15) as f32;
        }
        */

    }
}
