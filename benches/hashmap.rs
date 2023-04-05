use rand_distr::num_traits::pow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use SpatialBenches::Spatial;
use SpatialBenches::utilities::{random_points, uniform_position};

type Point2 = [f32; 3];
type IPoint2 = [i32; 3];

#[derive(Default)]
pub struct Benchmark {
    pub map: HashMap<IPoint2, Vec<Point2>>,
    pub list_offsets: Vec<IPoint2>,
    pub cell_size: f32,
}

impl Benchmark {
    fn insert(&mut self, global: Point2) {
        let local = self.global_to_map_loc(global);

        // Add entity to selected map cell
        match self.map.entry(local) {
            Entry::Occupied(mut o) => {
                o.get_mut().push(global);
            }
            Entry::Vacant(v) => {
                v.insert(vec![global]);
            }
        };
    }

    fn bulk_insert(&mut self, bulk: Vec<Point2>) {
        for p in bulk {
            self.insert(p);
        }
    }

    fn global_to_map_loc(&self, global: Point2) -> IPoint2 {
        let x = f32::floor(global[0] / self.cell_size) as i32;
        let y = f32::floor(global[1] / self.cell_size) as i32;
        let z = f32::floor(global[2] / self.cell_size) as i32;
        return [x, y, z];
    }
}

impl Spatial for Benchmark {
    fn build_tree(&mut self, count: i32) {
        let pts = random_points(count);
        self.bulk_insert(pts);
    }

    fn nearest(&mut self) {

    }

    fn within(&mut self, range: f32) {
        let mut list = Vec::new();
        let local = self.global_to_map_loc(uniform_position());

        // Broad range checks
        for offset in self.list_offsets.iter() {
            let key = add(local, *offset);

            if let Some(tfs) = self.map.get(&key) {
                list.extend(tfs.clone());
            }
        }

        // Precise range check
        let perception_squared = pow(range, 2);

        let mut temp_list = Vec::new();
        for pos in list {
            let origin = [0.0, 0.0];

            let distance_squared = pow(pos[0] - origin[0], 2) + pow(pos[1] - origin[1], 2);
            if distance_squared <= perception_squared {
                temp_list.push(pos);
            }
        }
    }
}

fn add(lhs: IPoint2, rhs: IPoint2) -> IPoint2 {
    return [lhs[0] + rhs[0], lhs[1] + rhs[1], lhs[2] + rhs[2]];
}
