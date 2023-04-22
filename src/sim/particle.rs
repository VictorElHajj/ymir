use bevy::prelude::*;

pub struct Particle {
    pub pos: Vec2,
    pub dir: Vec2,
    pub velocity: f32,
    pub water: f32,
    pub sediment: f32,
}

impl Particle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            dir: Vec2::ZERO,
            velocity: 1.0,
            water: 1.0,
            sediment: 0.0,
        }
    }
}
