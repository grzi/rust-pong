use amethyst::core::{
	timing::Time,
	SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::ball::Ball;
use crate::paddle::{Paddle, Side, PADDLE_WIDTH};
use crate::pong::ARENA_WIDTH;

#[derive(SystemDesc)]
pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, Ball>,  Read<'s, Time>);

    fn run(&mut self, (mut transforms, balls, time): Self::SystemData) {
       for (ball, transform) in (&balls, &mut transforms).join() {
		   	transform.prepend_translation_x(ball.velocity.0 * time.delta_seconds());
            transform.prepend_translation_y(ball.velocity.1 * time.delta_seconds());
	   }

    }
}
