use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::ball::{Ball, BALL_WIDTH, BALL_HEIGHT};
use crate::pong::ARENA_WIDTH;
use crate::paddle::{Paddle, PADDLE_HEIGHT, PADDLE_WIDTH, Side};

#[derive(SystemDesc)]
pub struct BallBounceSystem;

impl<'s> System<'s> for BallBounceSystem {
	
	type SystemData = 	(
		WriteStorage<'s, Ball>,
		ReadStorage<'s, Paddle>,
		ReadStorage<'s, Transform>
	);

	fn run(&mut self, (mut balls, paddles, transforms): Self::SystemData) {
		for (mut ball, transform) in (&mut balls, &transforms).join() {
			
			let (ball_x, ball_y) = (transform.translation().x, transform.translation().y);
			
			// wall bounce :
			if (ball_x + BALL_WIDTH) >= ARENA_WIDTH || ball_x <= 0.0 {
				ball.velocity.0 = -ball.velocity.0;
			}
			
			for (paddle, transform) in (&paddles, &transforms).join() {
				if ball_touch_paddle(ball_x, ball_y, transform.translation().x - (PADDLE_WIDTH * 0.5), transform.translation().y - (PADDLE_HEIGHT * 0.5)) {
					if (paddle.side == Side::Top && ball.velocity.1 < 0.0)
                        || (paddle.side == Side::Bottom && ball.velocity.1 > 0.0)
                    {
						ball.velocity.1 = -ball.velocity.1;
						if ball.velocity.1 > 0.0 {
							ball.velocity.1 += 5.0;
						}else{
							ball.velocity.1 -= 5.0;
						}

						ball.velocity.0 = ball.velocity.0 + 5.0;

					}
				}
			}
		}
	}
}

fn ball_touch_paddle(ball_x: f32, ball_y: f32, paddle_start_x: f32, paddle_start_y: f32) -> bool{
	// collision area
	let (min_x, min_y, max_x, max_y) = (
		paddle_start_x - BALL_WIDTH, paddle_start_y - BALL_HEIGHT,
		paddle_start_x + PADDLE_WIDTH + BALL_WIDTH, paddle_start_y + PADDLE_HEIGHT + BALL_HEIGHT
	);

	ball_x > min_x && ball_x < max_x && ball_y > min_y && ball_y < max_y
}
