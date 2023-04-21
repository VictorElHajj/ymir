use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sim::simulation::{simulate_drops, Simulation};
use sim::terrain::*;

fn criterion_benchmark(c: &mut Criterion) {
    let sim = Simulation {
        inertia: 0.4,
        capacity: 0.5,
        min_slope: 0.5,
        erosion: 0.5,
        gravity: 0.5,
        evaporation: 0.5,
        max_steps: 500,
        radius: 0.5,
    };
    let mut terrain = Terrain::new();
    c.bench_function("Simulate drops (1000)", |b| {
        b.iter(|| simulate_drops(&sim, &mut terrain, 1000))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
