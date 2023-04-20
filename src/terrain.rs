use bevy::prelude::{Commands, Resource, Vec2};
use bracket_noise::prelude::*;
use rand::Rng;

pub const WIDTH: usize = 240;
pub const HEIGHT: usize = 180;
pub const SCALE_FACTOR: f32 = 1.0;

#[derive(Resource)]
pub struct Terrain {
    pub map: [[f32; WIDTH]; HEIGHT],
    trace: [[bool; WIDTH]; HEIGHT],
}

impl Terrain {
    /// New initial terrain from noise
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut noise = FastNoise::seeded(rng.gen::<u64>());
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(2.0);
        noise.set_frequency(2.0);

        let map = &mut [[0.0; WIDTH]; HEIGHT];
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                // + 0.5 to make range 0.0 - 1.0
                let perlin =
                    noise.get_noise(i as f32 / HEIGHT as f32, j as f32 / WIDTH as f32) + 0.5;

                /*
                let distance_center = f32::sqrt(
                    f32::powi(usize::abs_diff(j, WIDTH / 2) as f32, 2)
                        + f32::powi(usize::abs_diff(i, HEIGHT / 2) as f32, 2),
                ) / f32::sqrt((WIDTH.pow(2) + HEIGHT.pow(2)) as f32);
                */

                // "Water level", with 0.4 ~ 60% of the world should be land
                map[i][j] = if perlin < 0.0 { 0.0 } else { perlin };
            }
        }
        Terrain {
            map: *map,
            trace: [[false; WIDTH]; HEIGHT],
        }
    }

    /// Is float position in bounds?
    pub fn inside(&self, pos: Vec2) -> bool {
        return pos.x >= 1.
            && pos.x <= ((WIDTH - 1) as f32)
            && pos.y >= 1.
            && pos.y <= ((HEIGHT - 1) as f32);
    }

    /// Returns neighbors
    pub fn neighbors(&self, pos: Vec2) -> [&f32; 8] {
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;
        [
            &self.map[y - 1][x - 1],
            &self.map[y - 1][x],
            &self.map[y - 1][x + 1],
            &self.map[y][x - 1],
            &self.map[y][x + 1],
            &self.map[y + 1][x - 1],
            &self.map[y + 1][x],
            &self.map[y + 1][x + 1],
        ]
    }

    /// Iterate over map cells and fill frame with height value converted to RBGA
    pub fn height_map(&self, frame: &mut [u8]) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let frame_loc = y * WIDTH * 4 + x * 4;
                let rgba_slice: &mut [u8] = &mut frame[frame_loc..frame_loc + 4];
                let color = (cell * 255.0) as u8;
                rgba_slice.copy_from_slice(&[color, color, color, 255]);
            }
        }
    }

    pub fn draw_trace(&self, frame: &mut [u8]) {
        for (y, row) in self.trace.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let frame_loc = y * WIDTH * 4 + x * 4;
                let rgba_slice: &mut [u8] = &mut frame[frame_loc..frame_loc + 4];
                if let true = cell {
                    rgba_slice.copy_from_slice(&[0, 0, 255, 255]);
                }
            }
        }
    }

    pub fn clear_trace(&mut self) {
        self.trace = [[false; WIDTH]; HEIGHT];
    }

    pub fn set_trace(&mut self, pos: Vec2) {
        self.trace[pos.y as usize][pos.x as usize] = true;
    }
}

pub fn setup_terrain(mut cmd: Commands) {
    cmd.insert_resource(Terrain::new());
}
