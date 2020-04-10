use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::ball::Ball;
use crate::paddle::{Paddle, Side, PADDLE_WIDTH};
use crate::pong::ARENA_WIDTH;

#[derive(SystemDesc)]
pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, Ball>);

    fn run(&mut self, (mut transforms, ball): Self::SystemData) {
        println!("coucou");
    }
}
