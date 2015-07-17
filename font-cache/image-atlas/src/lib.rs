extern crate rustc_serialize;
extern crate image;
extern crate fontcache;

use fontcache::RenderedFont;
use rustc_serialize::json;
use std::io::{Read, Write};
use std::path::Path;
use std::fs::File;
use std::convert::AsRef;
use image::ImageFormat;

pub use error::*;

mod error;

pub fn load_atlas(image: &[u8], metadata: &str)
-> DecodingResult<RenderedFont<image::DynamicImage>> {
    let img = try!(image::load_from_memory(image)
                         .map_err(DecodingError::ImageDecodingError));
    let meta: RenderedFont<()> = try!(json::decode(metadata));
    Ok(meta.map_img(move |_| (img, ())).0)
}

pub fn read_atlas<R1, R2>(image: &mut R1, metadata: &mut R2)
-> DecodingResult<RenderedFont<image::DynamicImage>>
where R1: Read, R2: Read {
    let mut image_bytes = Vec::new();
    let mut metadata_str = String::new();
    try!(image.read_to_end(&mut image_bytes));
    try!(metadata.read_to_string(&mut metadata_str));
    load_atlas(&image_bytes[..], &metadata_str[..])
}


pub fn save_atlas<P1, P2>(rendered: RenderedFont<image::DynamicImage>,
                          format: ImageFormat,
                          image: P1,
                          metadata: P2) -> EncodingResult<()>
where P1: AsRef<Path>, P2: AsRef<Path>
{
    let mut img_file = try!(File::open(image));
    let mut meta_file = try!(File::open(metadata));
    write_atlas(rendered, format, &mut img_file, &mut meta_file)
}

pub fn write_atlas<W1, W2>(rendered: RenderedFont<image::DynamicImage>,
                           format: ImageFormat,
                           image: &mut W1,
                           metadata: &mut W2) -> EncodingResult<()>
where W1: Write, W2: Write
{
    let (meta, img) = rendered.map_img(|i| ((), i));
    let _ = try!(img.save(image, format));
    let encoded = try!(json::encode(&meta));
    try!(metadata.write_all(encoded.as_bytes()));
    Ok(())
}
