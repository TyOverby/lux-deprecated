#[derive(Copy, Show, Clone)]
#[vertex_format]
pub struct ColorVertex {
    pub pos: [f32; 2],
    pub color: [f32; 4],
}

#[uniforms]
pub struct ColorParams {
    pub matrix: [[f32; 4]; 4],
}

//
//
// BASIC
//
//

pub static VERTEX_SRC: &'static str =
"
    #version 110

    uniform mat4 matrix;

    attribute vec2 pos;
    attribute vec4 color;

    varying vec4 v_color;

    void main() {
        gl_Position = matrix * vec4(pos, 0.0, 1.0);
        v_color = color;
    }
";

pub static FRAGMENT_SRC: &'static str =
"
    #version 110
    varying vec4 v_color;

    void main() {
        gl_FragColor = v_color;
    }
";
