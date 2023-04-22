use bevy::prelude::{Commands, Resource, Vec2};
use bracket_noise::prelude::*;
use rand::Rng;

pub const WIDTH: usize = 1600;
pub const HEIGHT: usize = 800;
pub const SCALE_FACTOR: f32 = 1.0;

type Matrix<T> = Box<[[T; WIDTH]; HEIGHT]>;

#[derive(Resource)]
pub struct Terrain {
    pub map: Matrix<f32>,
    trace: Matrix<bool>,
}

impl Terrain {
    /// New initial terrain from noise
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut noise = FastNoise::seeded(rng.gen::<u64>());
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(7);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(1.7);
        noise.set_frequency(2.0);

        let mut map: Matrix<f32> = vec![[0.0; WIDTH]; HEIGHT]
            .into_boxed_slice()
            .try_into()
            .unwrap();
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
            map,
            trace: vec![[false; WIDTH]; HEIGHT]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
        }
    }

    /// Is float position in bounds?
    pub fn inside(&self, pos: Vec2) -> bool {
        // Will gradient check give index out of bounds?
        if pos.x as usize + 1 == WIDTH || pos.y as usize + 1 == HEIGHT {
            return false;
        }
        // Actual bounds check
        return pos.x >= 0. && pos.x <= (WIDTH as f32) && pos.y >= 0. && pos.y <= (HEIGHT as f32);
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

    /// Calculate gradient of float pos by bilinear interpolation
    pub fn gradient(&self, pos: &Vec2) -> Vec2 {
        // Let x and y be ints such that drop.pos = (x + u, y + v) where u and v are real
        let x = pos.x.floor() as usize;
        let u = pos.x.fract();
        let y = pos.y.floor() as usize;
        let v = pos.y.fract();

        let ne = self.map[y][x + 1];
        let se = self.map[y + 1][x + 1];
        let nw = self.map[y][x];
        let sw = self.map[y + 1][x];

        // Calculate gradient by bilinear interpolation
        Vec2::new(
            (ne - nw) * (1. - v) + (se - sw) * v,
            (sw - nw) * (1. - u) + (se - ne) * u,
        )
        .normalize()
    }

    /// Calculate height of float pos by bilinear interpolation
    pub fn height(&self, pos: &Vec2) -> f32 {
        // Let x and y be ints such that drop.pos = (x + u, y + v) where u and v are real
        let x = pos.x.floor() as usize;
        let u = pos.x.fract();
        let y = pos.y.floor() as usize;
        let v = pos.y.fract();

        let ne = self.map[y][x + 1];
        let se = self.map[y + 1][x + 1];
        let nw = self.map[y][x];
        let sw = self.map[y + 1][x];

        nw * (1. - u) * (1. - v) + ne * u * (1. - v) + sw * (1. - u) * v + se * u * v
    }

    /// Deposit sediment by bilinear interpolation
    pub fn deposit(&mut self, pos: &Vec2, deposit_amount: f32) {
        // Let x and y be ints such that drop.pos = (x + u, y + v) where u and v are real
        let x = pos.x.floor() as usize;
        let u = pos.x.fract();
        let y = pos.y.floor() as usize;
        let v = pos.y.fract();

        // NE
        self.map[y][x + 1] += deposit_amount * u * (1. - v);
        // SE
        self.map[y + 1][x + 1] += deposit_amount * u * v;
        // NW
        self.map[y][x] += deposit_amount * (1. - u) * (1. - v);
        // SW
        self.map[y + 1][x] += deposit_amount * (1. - u) * v;
    }

    /// Erode sediment from terrain cells within radius
    pub fn erode(&mut self, pos: &Vec2, deposit_amount: f32, radius: usize) {
        let origin_x = pos.x.floor() as usize;
        let origin_y = pos.y.floor() as usize;
        let mut positions: Vec<((usize, usize), f32)> = Vec::with_capacity((radius * 2 + 1).pow(2));
        let mut sum = 0.0;

        // Ternary to avoid subtracting with overflow
        let y_start = if radius > origin_y {
            0
        } else {
            origin_y - radius
        };
        let x_start = if radius > origin_x {
            0
        } else {
            origin_x - radius
        };
        for y in y_start..usize::min(HEIGHT, origin_y + radius) {
            for x in x_start..usize::min(WIDTH, origin_x + radius) {
                // Is cell within radius?
                if usize::abs_diff(origin_y, y).pow(2) + usize::abs_diff(origin_x, x).pow(2)
                    < radius.pow(2)
                {
                    let grid_pos = Vec2::new(x as f32, y as f32);
                    let dist_val = f32::max(0., radius as f32 - pos.distance(grid_pos));
                    sum += dist_val;
                    positions.push(((x, y), dist_val));
                }
            }
        }
        for ((x, y), dist_val) in positions {
            let weight = dist_val / sum;
            self.map[y][x] -= deposit_amount * weight;
        }
    }

    /// Iterate over map cells and fill frame with height value converted to RBGA
    pub fn height_map(&self, frame: &mut [u8]) {
        let mut max_height = f32::MIN;
        let mut min_height = f32::MAX;
        for row in self.map.iter() {
            for cell in row.iter() {
                max_height = cell.max(max_height);
                min_height = cell.min(min_height);
            }
        }

        let range = max_height - min_height;
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let frame_loc = y * WIDTH * 4 + x * 4;
                let rgba_slice: &mut [u8] = &mut frame[frame_loc..frame_loc + 4];
                let normalized_height = (cell - min_height) / range;
                let color = (normalized_height * 255.0) as u8;
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
        self.trace = vec![[false; WIDTH]; HEIGHT]
            .into_boxed_slice()
            .try_into()
            .unwrap();
    }

    pub fn set_trace(&mut self, pos: Vec2) {
        self.trace[pos.y as usize][pos.x as usize] = true;
    }
}

pub fn setup_terrain(mut cmd: Commands) {
    cmd.insert_resource(Terrain::new());
}
