use glium::texture::Texture2d;
use glium::uniforms;

/// A colored vertex.
#[derive(Copy, Debug, Clone)]
pub struct ColorVertex {
    /// The position in screen space.
    pub pos: [f32; 2],
    /// The color in [r, g, b, a].
    pub color: [f32; 4],
}
implement_vertex!(ColorVertex, pos, color);

/// A textured vertex.
///
/// `tex_coords` is the position on the texture
/// where x and y are in the range `0.0` to `1.0`.
#[derive(Copy, Debug, Clone)]
pub struct TexVertex {
    /// The position in screen space
    pub pos: [f32; 2],
    /// The texture cooordinates [x, y] where x and y
    /// are in the range `0.0` to `1.0`.
    pub tex_coords: [f32; 2]
}
implement_vertex!(TexVertex, pos, tex_coords);

pub struct ColorParams {
    pub matrix: [[f32; 4]; 4],
}

impl uniforms::Uniforms for ColorParams {
    fn visit_values<'b, F>(&'b self, mut f: F) where F: FnMut(&str, uniforms::UniformValue<'b>) {
        use glium::uniforms::AsUniformValue;
        f("matrix", self.matrix.as_uniform_value());
    }
}

pub struct TexParams<'a> {
    pub matrix: [[f32; 4]; 4],
    pub tex: &'a Texture2d,
    pub color_mult: [f32; 4]
}

impl <'a> uniforms::Uniforms for TexParams<'a> {
    fn visit_values<'b, F>(&'b self, mut f: F) where F: FnMut(&str, uniforms::UniformValue<'b>) {
        use glium::uniforms::AsUniformValue;
        f("matrix", self.matrix.as_uniform_value());
        f("tex", self.tex.as_uniform_value());
        f("color_mult", self.color_mult.as_uniform_value());
    }
}
