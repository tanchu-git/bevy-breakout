use bevy::{math::vec3, prelude::*};

use crate::{
    resources::Collider,
    walls::{LEFT_WALL, RIGHT_WALL, WALL_THICKNESS},
};

// Paddle details
const PADDLE_START_Y: f32 = -240.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_SPEED: f32 = 500.0;

#[derive(Component)]
pub(crate) struct Paddle;

impl Paddle {
    pub(crate) fn spawn_paddle(commands: &mut Commands, asset_server: &Res<AssetServer>) {
        // Load texture
        let paddle_texture = asset_server.load("textures/paddle.png");

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: vec3(0.0, PADDLE_START_Y, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    // color: PADDLE_COLOR,
                    custom_size: Some(PADDLE_SIZE),
                    ..default()
                },
                texture: paddle_texture,
                ..default()
            },
            Paddle,
            Collider { size: PADDLE_SIZE },
        ));
    }

    pub(crate) fn move_paddle(
        input: Res<Input<KeyCode>>,
        time_step: Res<Time>,
        mut query: Query<&mut Transform, With<Paddle>>,
    ) {
        let mut paddle_transform = query.single_mut();
        let mut direction = 0.0;

        if input.pressed(KeyCode::A) {
            direction -= 1.0;
        }

        if input.pressed(KeyCode::D) {
            direction += 1.0;
        }

        let mut x_pos =
            paddle_transform.translation.x + direction * PADDLE_SPEED * time_step.delta_seconds();

        x_pos = x_pos.min(RIGHT_WALL - (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
        x_pos = x_pos.max(LEFT_WALL + (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);

        paddle_transform.translation.x = x_pos;
    }
}
