use bevy::prelude::Resource;
use bracket_noise::prelude::*;
use rand::Rng;

pub const INITIAL_WIDTH: usize = 240;
pub const INITIAL_HEIGHT: usize = 180;
pub const SCALE_FACTOR: f32 = 1.0;

#[derive(Resource)]
pub struct Terrain {
    map: [[f32; INITIAL_WIDTH]; INITIAL_HEIGHT],
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

        let map = &mut [[0.0; INITIAL_WIDTH]; INITIAL_HEIGHT];
        for i in 0..INITIAL_HEIGHT {
            for j in 0..INITIAL_WIDTH {
                // + 0.5 to make range 0.0 - 1.0
                let perlin = noise.get_noise(
                    i as f32 / INITIAL_HEIGHT as f32,
                    j as f32 / INITIAL_WIDTH as f32,
                ) + 0.5;

                /*
                let distance_center =
                    f32::sqrt(
                        f32::powi(usize::abs_diff(j, INITIAL_WIDTH / 2) as f32, 2)
                            + f32::powi(usize::abs_diff(i, INITIAL_HEIGHT / 2) as f32, 2),
                    ) / f32::sqrt((INITIAL_WIDTH.pow(2) + INITIAL_HEIGHT.pow(2)) as f32);
                */

                // "Water level", with 0.4 ~ 60% of the world should be land
                map[i][j] = if perlin < 0.4 { 0.0 } else { perlin };
            }
        }
        Terrain { map: *map }
    }

    /// Iterate over map cells and fill frame with height value converted to RBGA
    pub fn height_map(&self, frame: &mut [u8]) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let frame_loc = y * INITIAL_WIDTH * 4 + x * 4;
                let rgba_slice: &mut [u8] = &mut frame[frame_loc..frame_loc + 4];
                let color = (cell * 255.0) as u8;
                rgba_slice.copy_from_slice(&[color, color, color, 255]);
            }
        }
    }
}
