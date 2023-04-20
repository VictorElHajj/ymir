use bevy::prelude::*;
use rand::Rng;

use crate::terrain::{Terrain, HEIGHT, WIDTH};

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
        max_steps: 10,
        radius: 0.5,
    });
}

pub fn trace_drop(sim: Res<Simulation>, mut terrain: ResMut<Terrain>) {
    let mut rng = rand::thread_rng();
    let start_pos = Vec2::new(
        rng.gen_range(0.0..WIDTH as f32),
        rng.gen_range(0.0..HEIGHT as f32),
    );
    terrain.set_trace(start_pos);
}
