pub mod utilities;

pub trait Spatial {
    fn build_tree(&mut self, count: i32);
    fn nearest(&mut self);
    fn within(&mut self, range: f32);
}
