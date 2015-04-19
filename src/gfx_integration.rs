use glium::texture::Texture2d;
use glium::uniforms;

// Colored Vertex
#[derive(Copy, Debug, Clone)]
pub struct ColorVertex {
    pub pos: [f32; 2],
    pub color: [f32; 4],
}
implement_vertex!(ColorVertex, pos, color);

// Textured Vertex
#[derive(Copy, Debug, Clone)]
pub struct TexVertex {
    pub pos: [f32; 2],
    pub tex_coords: [f32; 2]
}
implement_vertex!(TexVertex, pos, tex_coords);

pub struct ColorParams {
    pub matrix: [[f32; 4]; 4],
}

// TODO: use implement_uniforms!() here instead.
impl uniforms::Uniforms for ColorParams {
    fn visit_values<'b, F>(&'b self, mut f: F) where F: FnMut(&str, uniforms::UniformValue<'b>) {
        use glium::uniforms::AsUniformValue;
        f("matrix", self.matrix.as_uniform_value());
    }
}

pub struct TexParams<'a> {
    pub matrix: [[f32; 4]; 4],
    pub texture: &'a Texture2d,
    pub color_mult: [f32; 4]
}

// TODO: use implement_uniforms!() here instead.
impl <'a> uniforms::Uniforms for TexParams<'a> {
    fn visit_values<'b, F>(&'b self, mut f: F) where F: FnMut(&str, uniforms::UniformValue<'b>) {
        use glium::uniforms::AsUniformValue;
        f("matrix", self.matrix.as_uniform_value());
        f("texture", self.texture.as_uniform_value());
        f("color_mult", self.color_mult.as_uniform_value());
    }
}
