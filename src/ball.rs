use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub const BALL_WIDTH: f32 = 4.0;
pub const BALL_HEIGHT: f32 = 4.0;
pub const BALL_VELOCITY_X: f32 = 50.0;
pub const BALL_VELOCITY_Y: f32 = 45.0;
pub const BALL_RADIUS: f32 = 1.0;


pub struct Ball {
	pub velocity: (f32, f32),
	pub radius: f32
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
			velocity: (BALL_VELOCITY_X, BALL_VELOCITY_Y),
			radius: BALL_RADIUS
        }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
