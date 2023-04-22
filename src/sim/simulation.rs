use bevy::prelude::*;
use rand::Rng;

use crate::particle::*;
use crate::terrain::*;

// Contains the hydraulic erosion settings
#[derive(Resource)]
pub struct Simulation {
    // The blend value between old direction and gradient
    pub inertia: f32,
    // The amount of sediment a drop is capabale of holding.
    pub capacity: f32,
    // Used to prevent capacity from reaching 0 in flat areas if desired.
    pub min_slope: f32,
    // Speed of erosion
    pub erosion: f32,
    // Gravity, self explanetory
    pub gravity: f32,
    // How fast water evaporates
    pub evaporation: f32,
    // How many steps max to simulate each drop
    pub max_steps: u32,
    // The radius of erosion around the drop
    pub radius: usize,
    // How much sediment to drop when over carrying capacity
    pub deposition: f32,
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
        capacity: 8.,
        deposition: 0.2,
        erosion: 0.7,
        evaporation: 0.02,
        gravity: 10.,
        inertia: 0.3,
        max_steps: 64,
        min_slope: 0.01,
        radius: 4,
    });
}

pub fn simulate_drops(sim: &Simulation, terrain: &mut Terrain, drops: usize) {
    let mut rng = rand::thread_rng();

    terrain.clear_trace();
    for _ in 0..drops {
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

            let gradient = terrain.gradient(&drop.pos);
            let height_old = terrain.height(&drop.pos);

            // New direction depends on old direction and gradient depending on inertia
            drop.dir = (drop.dir * sim.inertia - gradient * (1. - sim.inertia)).normalize();
            let pos_old = drop.pos;
            drop.pos += drop.dir;

            // Is drop's new position outside bounds or at edges?
            if !terrain.inside(drop.pos) {
                break;
            }

            let height_new = terrain.height(&drop.pos);
            let height_delta = height_old - height_new;

            // Drop moved uphill, deposit sediment up to new height
            if height_delta < 0. {
                let deposited_sediment = f32::min(drop.sediment, height_delta);
                terrain.deposit(&pos_old, deposited_sediment);
                drop.sediment -= deposited_sediment;
            } else {
                // Drop sediment carrying capacity
                let carrying_capacity = f32::max(height_delta, sim.min_slope)
                    * drop.velocity
                    * drop.water
                    * sim.capacity;
                // Drop moved downhill
                if drop.sediment > carrying_capacity {
                    // Drop has more sediment than carrying capacity, drop
                    let deposited_sediment = (drop.sediment - carrying_capacity) * sim.deposition;
                    terrain.deposit(&pos_old, deposited_sediment);
                    drop.sediment -= deposited_sediment;
                } else {
                    // Drop has less sediment than carrying capacity, gather
                    let eroded_sediment = f32::min(
                        (carrying_capacity - drop.sediment) * sim.erosion,
                        height_delta,
                    );
                    terrain.erode(&pos_old, eroded_sediment, sim.radius);
                    drop.sediment += eroded_sediment;
                }
            }

            // Update velocity and evaporate water

            drop.velocity = f32::sqrt(f32::max(
                0.,
                drop.velocity.powi(2) + height_delta * sim.gravity,
            ));
            drop.water = drop.water * (1. - sim.evaporation);

            terrain.set_trace(drop.pos);
        }
    }
}

pub fn drop_system(sim: Res<Simulation>, mut terrain: ResMut<Terrain>) {
    simulate_drops(&sim, &mut terrain, 100);
}
