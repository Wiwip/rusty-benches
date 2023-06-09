use rstar::RTree;
use SpatialBenches::Spatial;
use SpatialBenches::utilities::{random_points, uniform_position};

#[derive(Default)]
pub struct Benchmark {
    tree: RTree<[f32; 3]>,
}

impl Benchmark {
    pub fn new() -> Self {
        let tree = RTree::new();

        Self { tree }
    }
}

impl Spatial for Benchmark {
    fn build_tree(&mut self, count: i32) {
        let pts = random_points(count);
        self.tree = RTree::bulk_load(pts);
    }

    fn nearest(&mut self) {
        self.tree.nearest_neighbor(&uniform_position());
    }

    fn within(&mut self, range: f32) {
        self.tree.locate_within_distance(uniform_position(), range);
    }
}
