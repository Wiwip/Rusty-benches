use rand::distributions::Uniform;
use rand::prelude::*;
use rand::thread_rng;

pub fn random_points(count: i32) -> Vec<[f32; 3]> {
    let mut list = Vec::new();

    for _ in 0..count {
        list.push(uniform_position());
    }

    return list;
}

pub fn uniform_position() -> [f32; 3] {
    let mut rng = thread_rng();
    let mut dist = Uniform::from(-1000.0..1000.0);

    let pos = [
        dist.sample(&mut rng),
        dist.sample(&mut rng),
        dist.sample(&mut rng),
    ];

    pos
}