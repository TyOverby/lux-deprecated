use glium;
use vecmath;

use std::rc::Rc;
use std::ops::Deref;

use super::{ImageError, Figure, LuxCanvas, PrimitiveCanvas, TexVertex};

pub struct Sprite {
    texture: Rc<glium::texture::Texture2d>,
    original_size: (u32, u32),

    size: (u32, u32),
    pos: (u32, u32),

    texture_size: (f32, f32),
    texture_pos: (f32, f32),
}

pub trait SpriteLoader {
    fn load_sprite(&mut self, path: &::std::path::Path) -> Result<Sprite, ImageError>;

    fn sprite_from_pixels(&mut self, Vec<Vec<[f32; 4]>>) -> Sprite;
}

impl Sprite {
    pub fn new(tex: Rc<glium::texture::Texture2d>) -> Sprite {
        use glium::Surface;
        let size = tex.as_surface().get_dimensions();
        Sprite {
            texture: tex,
            original_size: size,
            size: size,
            pos: (0, 0),

            texture_size: (1.0, 1.0),
            texture_pos: (0.0, 0.0)
        }
    }

    pub fn sub_sprite(&self, offset: (u32, u32), size: (u32, u32)) -> Option<Sprite> {
        if offset.0 + size.0 > self.size.0 { return None };
        if offset.1 + size.1 > self.size.1 { return None };

        let pos = (self.pos.0 + offset.0, self.pos.1 + offset.1);

        Some(Sprite {
            texture: self.texture.clone(),
            original_size: self.original_size,

            size: size,
            pos: pos,

            texture_size: (size.0 as f32 / self.original_size.0 as f32,
                           size.1 as f32 / self.original_size.1 as f32),
            texture_pos: (pos.0 as f32 / self.original_size.0 as f32,
                          pos.1 as f32 / self.original_size.1 as f32)
        })
    }

    pub fn original_sprite(&self) -> Sprite {
        Sprite {
            texture: self.texture.clone(),
            original_size: self.original_size,
            size: self.original_size,
            pos: (0, 0),
            texture_size: (1.0, 1.0),
            texture_pos: (0.0, 0.0)
        }
    }

    fn bounds(&self) -> [[f32; 2]; 4]{
        let top_left = [self.texture_pos.0,
                        self.texture_pos.1];
        let top_right = [self.texture_pos.0 + self.texture_size.0,
                         self.texture_pos.1];
        let bottom_left = [self.texture_pos.0,
                           self.texture_pos.1 + self.texture_size.1];
        let bottom_right= [self.texture_pos.0 + self.texture_size.0,
                           self.texture_pos.1 + self.texture_size.1];

        [top_left, top_right, bottom_left, bottom_right]
    }

    pub fn texture(&self) -> Rc<glium::texture::Texture2d> {
        self.texture.clone()
    }

    pub fn texture_ref(&self) -> &glium::texture::Texture2d {
        self.texture.deref()
    }
}

impl Figure for Sprite {
    fn draw<C: LuxCanvas>(&self, canvas: &mut C) {
        let [top_left, top_right, bottom_left, bottom_right] = self.bounds();

        let tex_vs = vec![
            TexVertex {pos: [1.0, 0.0], tex_coords: top_right},
            TexVertex {pos: [0.0, 0.0], tex_coords: top_left},
            TexVertex {pos: [0.0, 1.0], tex_coords: bottom_left},
            TexVertex {pos: [1.0, 1.0], tex_coords: bottom_right},
        ];

        let idxs = [0u32, 1, 2, 0, 2, 3];

        canvas.draw_tex(super::TrianglesList, &tex_vs[], Some(&idxs[]), None, self.texture(), None);
    }
}
