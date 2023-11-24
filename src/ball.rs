use crate::{resources::Collider, Block, CollisionSound, Scoreboard, Velocity};
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::Rng;

// Ball details
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BALL_STARTING_POS: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const BALL_SPEED: f32 = 400.0;
const _BALL_INIT_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

#[derive(Component)]
pub(crate) struct Ball {
    size: Vec2,
}

impl Ball {
    pub(crate) fn spawn_ball(commands: &mut Commands, asset_server: &Res<AssetServer>) {
        // Load texture
        let ball_texture = asset_server.load("textures/ball.png");

        let mut rng = rand::thread_rng();
        let random_direction = Vec2::new(rng.gen_range(-1.0..=1.0), -0.5);

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: BALL_STARTING_POS,
                    ..default()
                },
                sprite: Sprite {
                    color: BALL_COLOR,
                    custom_size: Some(BALL_SIZE),
                    ..default()
                },
                texture: ball_texture,
                ..default()
            },
            Ball { size: BALL_SIZE },
            Velocity(BALL_SPEED * random_direction),
        ));
    }

    pub(crate) fn ball_collision(
        mut commands: Commands,
        mut scoreboard: ResMut<Scoreboard>,
        mut ball_query: Query<(&mut Velocity, &Transform, &Ball)>,
        collider_query: Query<(Entity, &Transform, &Collider, Option<&Block>)>,
        collision_sound: Res<CollisionSound>,
    ) {
        for (mut ball_velocity, ball_transform, ball) in &mut ball_query {
            for (other_entity, transform, other, opt_block) in &collider_query {
                let collision = collide(
                    ball_transform.translation,
                    ball.size,
                    transform.translation,
                    other.size,
                );

                let mut reflect_x = false;
                let mut reflect_y = false;
                if let Some(collision) = collision {
                    match collision {
                        Collision::Left => reflect_x = ball_velocity.x > 0.0,
                        Collision::Right => reflect_x = ball_velocity.x < 0.0,
                        Collision::Top => reflect_y = ball_velocity.y < 0.0,
                        Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                        Collision::Inside => (),
                    }

                    if reflect_x {
                        ball_velocity.x *= -1.0;
                    }

                    if reflect_y {
                        ball_velocity.y *= -1.0;
                    }

                    if opt_block.is_some() {
                        commands.entity(other_entity).despawn();
                        scoreboard.score += 1;
                    }

                    // Play sound
                    commands.spawn(AudioBundle {
                        source: collision_sound.to_owned(),
                        settings: PlaybackSettings::DESPAWN,
                    });
                };
            }
        }
    }
}
