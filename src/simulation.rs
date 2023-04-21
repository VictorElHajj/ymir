use bevy::prelude::*;
use rand::Rng;

use crate::{
    particle::Particle,
    terrain::{Terrain, HEIGHT, WIDTH},
};

// Contains the hydraulic erosion settings
#[derive(Resource)]
pub struct Simulation {
    // The blend value between old direction and gradient
    inertia: f32,
    // The amount of sediment a drop is capabale of holding.
    capacity: f32,
    // Used to prevent capacity from reaching 0 in flat areas if desired.
    min_slope: f32,
    // Speed of erosion
    erosion: f32,
    // Gravity, self explanetory
    gravity: f32,
    // How fast water evaporates
    evaporation: f32,
    // How many steps max to simulate each drop
    max_steps: u32,
    // The radius of erosion around the drop
    radius: f32,
}

// For each drop:
// 1. Drop in random location
// 2. Until drop is at local minimum or below water level:
//      a) Calculate gradient
//      b) Blend gradient with old direction depending on inertia
//      c) New position is old position + new direction (|dir| == 1)
//      e) Calculate difference in height of old and new position height_delta
//      f) if height delta is positive the drop should drop sediment in old pos to fill pit (untill filled or sediment runs out)
//      g) if height_delta is negative, calculate new carrying capacity dpending on delta, velocity and water.
//      h) if drop carries more sediment than capacity, drop sediemtn dependign on p_erosion
//      i) speed is adjusted and some water is evaporated

pub fn setup_simulation(mut cmd: Commands) {
    cmd.insert_resource(Simulation {
        inertia: 0.5,
        capacity: 0.5,
        min_slope: 0.5,
        erosion: 0.5,
        gravity: 0.5,
        evaporation: 0.5,
        max_steps: 500,
        radius: 0.5,
    });
}

pub fn trace_drop(sim: Res<Simulation>, mut terrain: ResMut<Terrain>) {
    let mut rng = rand::thread_rng();

    //terrain.clear_trace();
    for _ in 0..1000 {
        // Will not spawn on edges
        // The 0.1 here are margins to avoid neighbor check falling out of bounds
        let start_pos = Vec2::new(
            rng.gen_range(0.0..(WIDTH - 2) as f32),
            rng.gen_range(0.0..(HEIGHT - 2) as f32),
        );

        terrain.set_trace(start_pos);

        let mut drop = Particle::new(start_pos);

        for _ in 0..=sim.max_steps {
            // Is drop outside bounds or at edges?
            if !terrain.inside(drop.pos) {
                break;
            }

            // Let x and y be ints such that drop.pos = (x + u, y + v) where u and v are real
            let x = drop.pos.x.floor() as usize;
            let u = drop.pos.x.fract();
            let y = drop.pos.y.floor() as usize;
            let v = drop.pos.y.fract();

            // Calculate gradient by bilinear interpolation of N, W, S and E neighbours.
            let gradient = Vec2::new(
                (terrain.map[y][x + 1] - terrain.map[y][x]) * (1. - v)
                    + (terrain.map[y + 1][x + 1] - terrain.map[y + 1][x]) * v,
                (terrain.map[y + 1][x] - terrain.map[y][x]) * (1. - u)
                    + (terrain.map[y + 1][x + 1] - terrain.map[y][x + 1]) * u,
            )
            .normalize();

            // New direction depends on old direction and gradient depending on inertia
            drop.dir = (drop.dir * sim.inertia - gradient * (1. - sim.inertia)).normalize();

            drop.pos += drop.dir;

            terrain.set_trace(drop.pos);
        }
    }
}
