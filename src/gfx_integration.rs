use gfx::ShaderSource;

#[deriving(Copy, Show)]
#[vertex_format]
pub struct Vertex {
    #[name = "a_Pos"]
    pub pos: [f32, ..2],
    #[name = "a_Color"]
    pub color: [f32, ..4],
    #[name = "a_TexCoord"]
    pub tex: [f32, ..2]
}

#[shader_param(BasicBatch)]
pub struct Params {
    #[name = "u_Transform"]
    pub transform: [[f32, ..4], ..4],
}

//
//
// BASIC
//
//

pub static VERTEX_SRC: ShaderSource<'static> = shaders! {
GLSL_120: b"
    #version 120
    attribute vec2 a_Pos;
    attribute vec4 a_Color;
    attribute vec2 a_TexCoord;
    varying vec2 v_TexCoord;
    varying vec4 v_Color;
    uniform mat4 u_Transform;
    void main() {
        v_TexCoord = a_TexCoord;
        v_Color = a_Color;
        gl_Position = u_Transform * vec4(a_Pos, 0.0, 1.0);
    }
"
GLSL_150: b"
    #version 150 core
    in vec2 a_Pos;
    in vec4 a_Color;
    in vec2 a_TexCoord;
    out vec2 v_TexCoord;
    uniform mat4 u_Transform;
    void main() {
        v_TexCoord = a_TexCoord;
        gl_Position = u_Transform * vec4(a_Pos, 0.0, 1.0);
    }
"
};

pub static FRAGMENT_SRC: ShaderSource<'static> = shaders! {
GLSL_120: b"
    #version 120
    varying vec2 v_TexCoord;
    varying vec4 v_Color;
    void main() {
        gl_FragColor = v_Color;
    }
"
GLSL_150: b"
    #version 150 core
    in vec2 v_TexCoord;
    in vec4 a_Color;
    out vec4 o_Color;
    void main() {
        o_Color = a_Color;
    }
"
};
