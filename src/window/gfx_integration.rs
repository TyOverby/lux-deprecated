use gfx::ShaderSource;

pub static VERTEX_SRC: ShaderSource = shaders! {
GLSL_120: b"
    #version 120
    attribute vec2 a_Pos;
    attribute vec3 a_Color;
    attribute vec2 a_Tex;
    varying vec4 v_Color;
    void main() {
        v_Color = vec4(a_Color, 1.0);
        gl_Position = vec4(a_Pos, 0.0, 1.0);
    }
"
GLSL_150: b"
    #version 150 core
    in vec2 a_Pos;
    in vec3 a_Color;
    in vec2 a_Tex;
    out vec4 v_Color;
    void main() {
        v_Color = vec4(a_Color, 1.0);
        gl_Position = vec4(a_Pos, 0.0, 1.0);
    }
"
};

pub static FRAGMENT_SRC: ShaderSource = shaders! {
GLSL_120: b"
    #version 120
    varying vec4 v_Color;
    void main() {
        gl_FragColor = v_Color;
    }
"
GLSL_150: b"
    #version 150 core
    in vec4 v_Color;
    out vec4 o_Color;
    void main() {
        o_Color = v_Color;
    }
"
};
