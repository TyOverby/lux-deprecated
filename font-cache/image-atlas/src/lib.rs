extern crate rustc_serialize;
extern crate image;
extern crate fontcache;
extern crate bincode;

use fontcache::RenderedFont;
use bincode::{encode_into};
use std::io::{Read, Write};
use std::path::Path;
use std::fs::File;
use std::convert::AsRef;
use image::ImageFormat;

pub use error::*;
mod error;

/// Load the image portion of a font atlas from a slice of bytes, and the
/// metadata portion from a slice.
pub fn load_atlas(image: &[u8], metadata: &[u8])
-> DecodingResult<RenderedFont<image::DynamicImage>> {
    let img = try!(image::load_from_memory(image)
                         .map_err(DecodingError::ImageDecodingError));
    let meta: RenderedFont<()> = try!(bincode::decode(metadata));
    Ok(meta.map(move |_| img))
}

/// Load the image portion of a font atlas from one Reader, and the
/// metadata portion from another.
pub fn read_atlas<R1, R2>(image: &mut R1, metadata: &mut R2)
-> DecodingResult<RenderedFont<image::DynamicImage>>
where R1: Read, R2: Read {
    let mut image_bytes = Vec::new();
    let mut metadata_bytes = Vec::new();
    try!(image.read_to_end(&mut image_bytes));
    try!(metadata.read_to_end(&mut metadata_bytes));
    load_atlas(&image_bytes[..], &metadata_bytes[..])
}


/// Saves an atlas to two paths.  One for the image (using the specified image format),
/// one for the font metadata.
pub fn save_atlas<P1, P2>(rendered: RenderedFont<image::DynamicImage>,
                          format: ImageFormat,
                          image: P1,
                          metadata: P2) -> EncodingResult<()>
where P1: AsRef<Path>, P2: AsRef<Path>
{
    let mut img_file = try!(File::create(image));
    let mut meta_file = try!(File::create(metadata));
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

    try!(bincode::encode_into(&just_meta, metadata, bincode::SizeLimit::Infinite));
    Ok(())
}
