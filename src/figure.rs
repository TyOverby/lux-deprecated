use ::prelude::{
    LuxCanvas,
    PrimitiveType,
    ColorVertex,
    TexVertex,
    Sprite
};

pub trait Figure {
    fn draw<C: LuxCanvas>(&self, &mut C);
}

impl <'a> Figure for (PrimitiveType, &'a[ColorVertex]) {
    fn draw<C: LuxCanvas>(&self, canvas: &mut C) {
        let &(ref p, vtxs) = self;
        canvas.draw_shape(*p, vtxs, None, None);
    }
}

impl <'a, 'b> Figure for (PrimitiveType, &'a[TexVertex], &'b Sprite) {
    fn draw<C: LuxCanvas>(&self, canvas: &mut C) {
        let &(ref p, vtxs, ref spr) = self;
        canvas.draw_tex(*p, vtxs, None, None, spr.texture(), None);
    }
}
