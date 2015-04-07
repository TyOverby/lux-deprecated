use typemap;
pub trait LuxExtend {
    fn typemap(&self) -> &typemap::TypeMap;
    fn typemap_mut(&mut self) -> &mut typemap::TypeMap;
}
