use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sim::simulation::{simulate_drops, Simulation};
use sim::terrain::*;

fn criterion_benchmark(c: &mut Criterion) {
    let sim = Simulation {
        capacity: 0.5,
        deposition: 0.2,
        erosion: 0.5,
        evaporation: 0.5,
        gravity: 0.5,
        inertia: 0.4,
        max_steps: 100,
        min_slope: 0.5,
        radius: 4,
    };
    let mut terrain = Terrain::new();
    c.bench_function("Simulate drops (1000)", |b| {
        b.iter(|| simulate_drops(&sim, &mut terrain, 1000))
    });

    let frame = &mut [0u8; WIDTH * HEIGHT * 4];
    c.bench_function("Fill pixel frame", |b| {
        b.iter(|| {
            terrain.height_map(frame);
            terrain.draw_trace(frame);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
