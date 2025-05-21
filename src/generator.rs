use rand::{Rng, SeedableRng};

fn generate_binpacking(items: usize, capacity: f64) -> Vec<f64> {
    let mut rng = rand::prelude::SmallRng::from_seed([0;  32]);
    let mut sizes = vec![];
    for _ in 0..items {
        sizes.push(rng.random_range(1.0..capacity));
    }
    sizes
}