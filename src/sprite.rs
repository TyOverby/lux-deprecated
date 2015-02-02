use glium;
use image;

use std::rc::Rc;
use std::ops::Deref;
use std::collections::hash_map::{HashMap, Hasher};
use std::borrow::BorrowFrom;

use std::cmp::Eq;
use std::hash::{Hash, SipHasher};

use super::{ImageError, TexVertex, Figure, LuxCanvas};

#[derive(Clone)]
pub struct Sprite {
    texture: Rc<glium::texture::Texture2d>,
    original_size: (u32, u32),

    size: (u32, u32),
    pos: (u32, u32),

    texture_size: (f32, f32),
    texture_pos: (f32, f32),
}

pub struct UniformSpriteSheet {
    sprite: Sprite,
    divs: (u32, u32),
    indiv_size: (u32, u32),
}

pub struct NonUniformSpriteSheet<K> {
    sprite: Sprite,
    mapping: HashMap<K, Sprite>
}

pub trait SpriteLoader {
    fn load_sprite(&mut self, path: &::std::path::Path) -> Result<Sprite, ImageError>;

    fn sprite_from_pixels(&mut self, Vec<Vec<[f32; 4]>>) -> Sprite;
    fn sprite_from_image(&mut self, img: image::DynamicImage) -> Sprite;
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

    pub fn ideal_size(&self) -> (f32, f32) {
        let (w, h) = self.size;
        (w as f32, h as f32)
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

    pub fn bounds(&self) -> [[f32; 2]; 4]{
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

    pub fn zeroed_vertices(&self) -> (Vec<TexVertex>, Vec<u32>) {
        let [top_left, top_right, bottom_left, bottom_right] = self.bounds();
        (
            vec![
                    TexVertex {pos: [1.0, 0.0], tex_coords: top_right},
                    TexVertex {pos: [0.0, 0.0], tex_coords: top_left},
                    TexVertex {pos: [0.0, 1.0], tex_coords: bottom_left},
                    TexVertex {pos: [1.0, 1.0], tex_coords: bottom_right},
             ],
             vec![0u32, 1, 2, 0, 2, 3]
        )
    }
    pub fn as_uniform_sprite_sheet(&self, indiv_width: u32, indiv_height: u32)
    -> UniformSpriteSheet {
        UniformSpriteSheet::new(self.clone(), indiv_width, indiv_height)
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

/// A sprite sheet that that is laid out in a grid where every grid is the
/// same height and width.
impl UniformSpriteSheet {
    /// Creates a new sprite-sheet that is divided into a `div_x` by `div_y`
    /// grid.
    fn new(sprite: Sprite, div_x: u32, div_y: u32) -> UniformSpriteSheet {
        let indiv_width = sprite.size.0 / div_x;
        let indiv_height = sprite.size.1 / div_y;
        UniformSpriteSheet{
            sprite: sprite,
            divs: (div_x, div_y),
            indiv_size: (indiv_width, indiv_height)
        }
    }

    /// Gets the sprite that is in the (x, y) position in the grid
    /// defined by this sprite sheet.
    ///
    /// ## Failure
    /// Fails if out of bounds.
    pub fn get(&self, x: u32, y: u32) -> Sprite {
        self.get_opt(x, y).unwrap()
    }

    /// Same as `get` but returns None instead of failing if
    /// the sprite is out of bounds.
    pub fn get_opt(&self, x: u32, y: u32) -> Option<Sprite> {
        let x_tex = x * self.indiv_size.0;
        let y_tex = y * self.indiv_size.1;

        self.sprite.sub_sprite((x_tex, y_tex), self.indiv_size)
    }
}

/// A non-uniform spritesheet is a sprite-sheet that is
/// indexable by arbitrary keys.
impl <K: Eq + Hash<Hasher>> NonUniformSpriteSheet<K> {
    /// Creates a new non-uniform spritesheet based off of this sprite.
    fn new(sprite: Sprite) -> NonUniformSpriteSheet<K> {
        NonUniformSpriteSheet {
            sprite: sprite,
            mapping: HashMap::new()
        }
    }

    /// Associates a key with a sprite location.
    fn associate(&mut self, key: K, pos: (u32, u32), size: (u32, u32)) {
        self.mapping.insert(key, self.sprite.sub_sprite(pos, size).unwrap());
    }

    /// Gets the sprite that is associated with a key.
    ///
    /// ## Failure
    /// Fails if the key doesn't associate to something yet.
    fn get<Q: ?Sized>(&mut self, key: &Q) -> Sprite
    where Q: Hash<Hasher> + Eq + BorrowFrom<K> {
        self.get_opt(key).unwrap()
    }

    /// Same as `get` but returns None instead of failing if the key
    /// doesn't associate to anything.
    fn get_opt<Q: ?Sized>(&mut self, key: &Q) -> Option<Sprite>
    where Q: Hash<Hasher> + Eq + BorrowFrom<K> {
        self.mapping.get(key).map(|a| a.clone())
    }
}
