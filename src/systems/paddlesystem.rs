use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::paddle::{Paddle, Side, PADDLE_WIDTH};
use crate::pong::ARENA_WIDTH;

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Top => input.axis_value("top_paddle"),
                Side::Bottom => input.axis_value("bottom_paddle"),
            };
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let side_name = match paddle.side {
                        Side::Top => "top",
                        Side::Bottom => "bottom",
                    };

                    let paddle_x = transform.translation().x;
                    let scaled_amount = 1.2 * mv_amount as f32;
                    let new_translation = (scaled_amount + paddle_x)
                        .max(PADDLE_WIDTH * 0.5)
                        .min(ARENA_WIDTH - PADDLE_WIDTH * 0.5);
                    transform.set_translation_x(new_translation);
                }
            }
        }
    }
}
