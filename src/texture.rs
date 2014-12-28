use device::tex::{
    TextureInfo,
    TextureKind,
    RGBA8,
    SamplerInfo,
};

use device::tex::FilterMethod::Bilinear;
use device::tex::WrapMode::Tile;

#[deriving(Copy)]
pub struct Texture {
    tex_info: TextureInfo,
    sampler: SamplerInfo
}

impl Texture {
    pub fn new(width: u16, height: u16) -> Texture {
        Texture {
            tex_info:
            TextureInfo {
                width: width,
                height: height,
                depth: 1,
                levels: -1,
                kind: TextureKind::Texture2D,
                format: RGBA8
            },
            sampler: SamplerInfo::new(Bilinear, Tile)
        }
    }
}
