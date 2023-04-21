use bevy::prelude::*;

pub struct Particle {
    pub pos: Vec2,
    pub dir: Vec2,
}

impl Particle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            dir: Vec2::ZERO,
        }
    }
}
