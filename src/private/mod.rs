pub mod interactive;
pub mod canvas;
pub mod raw;
pub mod gfx_integration;
pub mod glutin_window;
pub mod color;
pub mod sprite;
pub mod font;
pub mod error;
pub mod colors;
pub mod game;
pub mod accessors;
pub mod primitive_canvas;
pub mod shaders;
pub mod types;

pub mod constants {
    #[cfg(feature="freetype")]
    pub static SOURCE_CODE_PRO_REGULAR: &'static[u8] =
        include_bytes!("../../resources/SourceCodePro-Regular.ttf");

    // Rendered images
    pub static SCP_12_PNG: &'static[u8] =
        include_bytes!("../../resources/SourceCodePro-Regular-12.png");
    pub static SCP_20_PNG: &'static[u8] =
        include_bytes!("../../resources/SourceCodePro-Regular-20.png");
    pub static SCP_30_PNG: &'static[u8] =
        include_bytes!("../../resources/SourceCodePro-Regular-30.png");

    // Info files
    pub static SCP_12_BINCODE: &'static[u8] =
        include_bytes!("../../resources/SourceCodePro-Regular-12.bincode");
    pub static SCP_20_BINCODE: &'static[u8] =
        include_bytes!("../../resources/SourceCodePro-Regular-20.bincode");
    pub static SCP_30_BINCODE: &'static[u8] =
        include_bytes!("../../resources/SourceCodePro-Regular-30.bincode");
}
