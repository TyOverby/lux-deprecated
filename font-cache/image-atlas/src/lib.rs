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

/// Load the image portion of a font atlas from a slice of bytes, and the
/// metadata portion from a str.
pub fn load_atlas(image: &[u8], metadata: &str)
-> DecodingResult<RenderedFont<image::DynamicImage>> {
    let img = try!(image::load_from_memory(image)
                         .map_err(DecodingError::ImageDecodingError));
    let meta: RenderedFont<()> = try!(json::decode(metadata));
    Ok(meta.map(move |_| img))
}

/// Load the image portion of a font atlas from one Reader, and the
/// metadata portion from another.
pub fn read_atlas<R1, R2>(image: &mut R1, metadata: &mut R2)
-> DecodingResult<RenderedFont<image::DynamicImage>>
where R1: Read, R2: Read {
    let mut image_bytes = Vec::new();
    let mut metadata_str = String::new();
    try!(image.read_to_end(&mut image_bytes));
    try!(metadata.read_to_string(&mut metadata_str));
    load_atlas(&image_bytes[..], &metadata_str[..])
}


/// Saves an atlas to two paths.  One for the image (using the specified image format),
/// one for the font metadata.
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

/// Saves an atlas to two Writers.  One for the image (using the specified image format),
/// one for the font metadata.
pub fn write_atlas<W1, W2>(rendered: RenderedFont<image::DynamicImage>,
                           format: ImageFormat,
                           image: &mut W1,
                           metadata: &mut W2) -> EncodingResult<()>
where W1: Write, W2: Write
{
    let just_meta = rendered.map(|img| {
        img.save(image, format).map(|_| ())
    });
    let just_meta = try!(just_meta.reskin());

    let encoded = try!(json::encode(&just_meta));
    try!(metadata.write_all(encoded.as_bytes()));
    Ok(())
}
