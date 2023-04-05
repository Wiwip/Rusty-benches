use rapier2d::na::SMatrix;
use rapier2d::prelude::*;
use SpatialBenches::Spatial;
use SpatialBenches::utilities::random_points;


pub struct Benchmark {
    colliders: ColliderSet,
    rigid_bodies: RigidBodySet,
    island_manager: IslandManager,
    query_pipeline: QueryPipeline,
    physics_pipeline: PhysicsPipeline,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
}

impl Benchmark {
    pub fn new() -> Self {
        let bench = Benchmark {
            colliders: ColliderSet::new(),
            rigid_bodies: RigidBodySet::new(),
            island_manager: IslandManager::new(),
            query_pipeline: QueryPipeline::new(),

            physics_pipeline: PhysicsPipeline::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
        };

        bench
    }

    pub fn step(&mut self, steps: i32) {
        let integration_parameters = IntegrationParameters::default();
        let physics_hooks = ();
        let event_handler = ();
        let gravity = vector![0.0, -9.81];

        for i in 0..steps{
            self.physics_pipeline.step(
                &gravity,
                &integration_parameters,
                &mut self.island_manager,
                &mut self.broad_phase,
                &mut self.narrow_phase,
                &mut self.rigid_bodies,
                &mut self.colliders,
                &mut self.impulse_joint_set,
                &mut self.multibody_joint_set,
                &mut self.ccd_solver,
                None,
                &physics_hooks,
                &event_handler,
            );
        }
    }
}

impl Spatial for Benchmark {
    fn build_tree(&mut self, count: i32) {
        let points = random_points(count);

        for p in points {
            let body = RigidBodyBuilder::dynamic()
                .translation(vector![p[0], p[1]])
                .build();

            let handle = self.rigid_bodies.insert(body);

            let collider = ColliderBuilder::ball(1.0);
            self.colliders.insert_with_parent(collider, handle, &mut self.rigid_bodies);
        }
    }

    fn nearest(&mut self) {
        todo!()
    }

    fn within(&mut self, range: f32) {
        let shape = Ball::new(range);
        let shape_pos = Isometry::new(vector![0., 0.], 0.8);

        self.query_pipeline.intersections_with_shape(
            &self.rigid_bodies, &self.colliders, &shape_pos, &shape, QueryFilter::default(),
            |handle| {
                let count = 1;
                true
            });
    }
}
