use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::ball::{Ball, BALL_HEIGHT, BALL_WIDTH};
use crate::paddle::{Paddle, Side, PADDLE_HEIGHT};

pub struct Pong;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        initialize_paddles(world, sprite_sheet_handle.clone());
        initialize_ball(world, sprite_sheet_handle);
        initialize_camera(world);
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let asset_loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        asset_loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let asset_loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    asset_loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut top_transform = Transform::default();
    let mut bottom_transform = Transform::default();

    let x = ARENA_WIDTH / 2.0;

    top_transform.set_translation_xyz(x, PADDLE_HEIGHT * 0.5, 0.0);
    bottom_transform.set_translation_xyz(x, ARENA_HEIGHT - PADDLE_HEIGHT * 0.5, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Top))
        .with(top_transform)
        .build();

    world
        .create_entity()
        .with(sprite_render)
        .with(Paddle::new(Side::Bottom))
        .with(bottom_transform)
        .build();
}

fn initialize_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut ball_transform = Transform::default();

    let x = ARENA_WIDTH / 2.0 - BALL_WIDTH / 2.0;
    let y = ARENA_HEIGHT / 2.0 - BALL_HEIGHT / 2.0;

    ball_transform.set_translation_xyz(x, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball::new())
        .with(ball_transform)
        .build();
}
