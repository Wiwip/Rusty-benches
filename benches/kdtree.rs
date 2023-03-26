use kd_tree::KdTree;
use SpatialBenches::Spatial;
use SpatialBenches::utilities::random_points;

pub struct Benchmark {
    tree: KdTree<[f32; 3]>,
}

impl Benchmark {
    pub fn new() -> Self {
        let tree = KdTree::build_by_ordered_float(Vec::new());

        Self { tree }
    }
}

impl Spatial for Benchmark {
    fn build_tree(&mut self, count: i32) {
        let pts = random_points(count);
        self.tree = KdTree::build_by_ordered_float(pts);
    }

    fn nearest(&mut self) {
        self.tree.nearest(&[0.0, 0.0, 0.0]);
    }

    fn within(&mut self, range: f32) {
        self.tree.within_radius(&[0.0, 0.0, 0.0], range);
    }
}
