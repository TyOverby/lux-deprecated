extern crate fontcache;
extern crate glyph_packer;
extern crate image;
extern crate freetype;

use std::collections::HashMap;
use fontcache::{RenderedFont, CharInfo};

struct Advance(i32, i32);
struct BitmapOffset(i32, i32);

pub type FreetypeError = freetype::error::Error;
pub type FreetypeResult<T> = Result<T, FreetypeError>;

/// Given a freetype face, this function generates a RenderedFont with the
/// a `DynamicImage` backing it.
pub fn render<I: Iterator<Item=char>>(face: &mut freetype::Face, chars: I, kerning: bool)
-> FreetypeResult<fontcache::RenderedFont<image::DynamicImage>> {
    let chars_vec: Vec<_> = chars.collect();

    let (image, char_info) = try!(merge_all(chars_vec.iter().cloned().map(|c| (c, char_to_img(face, c)))));

    let line_height: Option<u32> = face.size_metrics().map(|m| m.height as u32 / 64);
    let line_height = try!(line_height.ok_or(freetype::error::Error::MissingProperty));

    let max_width: Option<u32> = face.size_metrics().map(|m| m.max_advance as u32 / 64);
    let max_width = try!(max_width.ok_or(freetype::error::Error::MissingProperty));

    let kerning = if kerning {
        let mut map = HashMap::new();
        for c1 in chars_vec.iter().cloned() {
            for c2 in chars_vec.iter().cloned() {
                let k = try!(face.get_kerning(
                    face.get_char_index(c1 as usize),
                    face.get_char_index(c2 as usize),
                    freetype::face::KerningMode::KerningDefault));
                let (dx, dy)  = (k.x, k.y);
                let (dx, dy) = (dx / 64, dy / 64);

                if dx != 0 || dy != 0 {
                    map.insert((c1, c2), (dx as i32, dy as i32));
                }
            }
        }
        map
    } else {
        HashMap::new()
    };

    Ok(RenderedFont::new(
        face.family_name(),
        face.style_name(),
        image,
        line_height,
        max_width,
        char_info,
        kerning
    ))
}

fn char_to_img(face: &freetype::Face, c: char) -> FreetypeResult<(image::DynamicImage, Advance, BitmapOffset)> {
    fn buf_to_vec(bf: &[u8], width: u32, height: u32) -> image::DynamicImage {
        let mut v = Vec::with_capacity((width * height * 2) as usize);
        for &p in bf {
            v.push(p);
            v.push(p);
        }
        image::DynamicImage::ImageLumaA8(
            image::ImageBuffer::from_vec(width, height, v).unwrap())
    }

    try!(face.load_char(c as usize, freetype::face::RENDER));

    let glyph = face.glyph();
    let bit = glyph.bitmap();

    let advance = glyph.advance();
    let advance = Advance(advance.x as i32 / 64, advance.y as i32 / 64);

    let offset = BitmapOffset(glyph.bitmap_left() as i32, glyph.bitmap_top() as i32);

    Ok((buf_to_vec(bit.buffer(), bit.width() as u32, bit.rows() as u32), advance, offset))
}

fn merge_all<I>(images: I) -> FreetypeResult<(image::DynamicImage, HashMap<char, CharInfo>)>
where I: Iterator<Item=(char, FreetypeResult<(image::DynamicImage, Advance, BitmapOffset)>)>,
      {
    use glyph_packer::{Packer, GrowingPacker};

    let size = 256u32;
    let mut packer: glyph_packer::SkylinePacker<_> = {
        let bf = image::DynamicImage::new_luma_a8(size, size);
        Packer::new(bf)
    };

    let mut mapping = HashMap::new();
    packer.set_margin(5);

    for (chr, comp) in images {
        let (img, adv, offset) = try!(comp);
        let rect = packer.pack_resize(&img, |(x, y)| (x * 2, y * 2));

        let ci = CharInfo {
            image_position: (rect.x, rect.y),
            image_size: (rect.w, rect.h),
            advance: (adv.0, adv.1),
            pixel_offset: (offset.0, offset.1)
        };
        mapping.insert(chr, ci);
    }

    let buf = packer.into_buf();
    Ok((buf, mapping))
}
