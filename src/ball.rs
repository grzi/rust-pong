use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub const BALL_WIDTH: f32 = 4.0;
pub const BALL_HEIGHT: f32 = 4.0;

pub struct Ball {
    pub width: f32,
    pub height: f32,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            width: BALL_WIDTH,
            height: BALL_HEIGHT,
        }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
