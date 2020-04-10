use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub const PADDLE_HEIGHT: f32 = 4.0;
pub const PADDLE_WIDTH: f32 = 16.0;

#[derive(PartialEq, Eq)]
pub enum Side {
    Top,
    Bottom,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
