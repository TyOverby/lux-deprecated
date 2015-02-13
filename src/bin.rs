#![allow(unstable)]
extern crate lux;
extern crate glium;
extern crate image;
extern crate freetype;

use lux::*;
use std::old_path::Path;
use std::old_io::File;

fn main() {
    let mut lux = Window::new().unwrap();


    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::BLACK);
    }
}
