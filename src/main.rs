use ball::Ball;
use bevy::{
    math::{vec2, vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use paddle::Paddle;

mod ball;
mod paddle;

// Border wall
const LEFT_WALL: f32 = -450.0;
const RIGHT_WALL: f32 = 450.0;
const BOTTOM_WALL: f32 = -300.0;
const TOP_WALL: f32 = 300.0;

const WALL_THICKNESS: f32 = 10.0;
const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
const WALL_BLOCK_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

// Blocks
const BLOCK_SIZE: Vec2 = Vec2::new(100.0, 30.0);
const BLOCK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const GAP_BETWEEN_PADDLE_AND_BLOCKS: f32 = 270.0;
const GAP_BETWEEN_BLOCKS: f32 = 5.0;
const GAP_BETWEEN_BLOCKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BLOCKS_AND_SIDES: f32 = 20.0;

// Scoreboard
const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Scoreboard { score: 0 })
        .add_systems(Update, (bevy::window::close_on_esc, update_scoreboard))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                Paddle::move_paddle,
                apply_velocity,
                Ball::ball_collision.after(apply_velocity),
            ),
        )
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider {
    size: Vec2,
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

#[derive(Component)]
struct Block;

#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

#[derive(Resource, Default, Deref, DerefMut)]
struct CollisionSound(Handle<AudioSource>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Load sound resources
    let collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(collision_sound));

    // Spawn paddle
    Paddle::spawn_paddle(&mut commands, &asset_server);

    // // Load texture and spawn ball
    // let ball_texture = asset_server.load("textures/ball.png");

    // commands.spawn((
    //     SpriteBundle {
    //         transform: Transform {
    //             translation: BALL_STARTING_POS,
    //             ..default()
    //         },
    //         sprite: Sprite {
    //             color: BALL_COLOR,
    //             custom_size: Some(BALL_SIZE),
    //             ..default()
    //         },
    //         texture: ball_texture,
    //         ..default()
    //     },
    //     Ball { size: BALL_SIZE },
    //     Velocity(BALL_SPEED * BALL_INIT_DIRECTION),
    // ));
    Ball::spawn_ball(&mut commands, &asset_server);

    // Spawn left wall
    let vertical_wall_size = vec2(WALL_THICKNESS, WALL_BLOCK_HEIGHT + WALL_THICKNESS);
    let horizontal_wall_size = vec2(WALL_BLOCK_WIDTH + WALL_THICKNESS, WALL_THICKNESS);

    commands.spawn(WallBundle {
        sprite_bundle: SpriteBundle {
            transform: Transform {
                translation: vec3(LEFT_WALL, 0.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(vertical_wall_size),
                ..default()
            },
            ..default()
        },
        collider: Collider {
            size: vertical_wall_size,
        },
    });

    // Spawn right wall
    commands.spawn(WallBundle {
        sprite_bundle: SpriteBundle {
            transform: Transform {
                translation: vec3(RIGHT_WALL, 0.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(vertical_wall_size),
                ..default()
            },
            ..default()
        },
        collider: Collider {
            size: vertical_wall_size,
        },
    });

    // Spawn bottom wall
    commands.spawn(WallBundle {
        sprite_bundle: SpriteBundle {
            transform: Transform {
                translation: vec3(0.0, BOTTOM_WALL, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(horizontal_wall_size),
                ..default()
            },
            ..default()
        },
        collider: Collider {
            size: horizontal_wall_size,
        },
    });

    // Spawn top wall
    commands.spawn(WallBundle {
        sprite_bundle: SpriteBundle {
            transform: Transform {
                translation: vec3(0.0, TOP_WALL, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(horizontal_wall_size),
                ..default()
            },
            ..default()
        },
        collider: Collider {
            size: horizontal_wall_size,
        },
    });

    // Blocks
    let offset_x = LEFT_WALL + GAP_BETWEEN_BLOCKS_AND_SIDES + BLOCK_SIZE.x * 0.5;
    let offset_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_BLOCKS + BLOCK_SIZE.y * 0.5;

    let blocks_total_width = (RIGHT_WALL - LEFT_WALL) - 2.0 * GAP_BETWEEN_BLOCKS_AND_SIDES;
    let blocks_total_height =
        (TOP_WALL - BOTTOM_WALL) - GAP_BETWEEN_BLOCKS_AND_CEILING - GAP_BETWEEN_PADDLE_AND_BLOCKS;

    let rows = (blocks_total_height / (BLOCK_SIZE.y + GAP_BETWEEN_BLOCKS)).floor() as i32;
    let columns = (blocks_total_width / (BLOCK_SIZE.x + GAP_BETWEEN_BLOCKS)).floor() as i32;

    for row in 0..rows {
        for column in 0..columns {
            let brick_pos = vec2(
                offset_x + column as f32 * (BLOCK_SIZE.x + GAP_BETWEEN_BLOCKS),
                offset_y + row as f32 * (BLOCK_SIZE.y + GAP_BETWEEN_BLOCKS),
            );

            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: brick_pos.extend(0.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: BLOCK_COLOR,
                        custom_size: Some(BLOCK_SIZE),
                        ..default()
                    },
                    ..default()
                },
                Block,
                Collider { size: BLOCK_SIZE },
            ));
        }
    }

    // Spawn scoreboard UI
    commands.spawn((TextBundle::from_sections([
        TextSection::new(
            "Score: ",
            TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: TEXT_COLOR,
                ..default()
            },
        ),
        TextSection::from_style(TextStyle {
            font_size: SCOREBOARD_FONT_SIZE,
            color: SCORE_COLOR,
            ..default()
        }),
    ])
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: SCOREBOARD_TEXT_PADDING,
        left: SCOREBOARD_TEXT_PADDING,
        ..default()
    }),));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time>) {
    let delta_time = time_step.delta_seconds();

    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * delta_time;
        transform.translation.y += velocity.y * delta_time;
    }
}

// fn ball_collision(
//     mut commands: Commands,
//     mut scoreboard: ResMut<Scoreboard>,
//     mut ball_query: Query<(&mut Velocity, &Transform, &Ball)>,
//     collider_query: Query<(Entity, &Transform, &Collider, Option<&Block>)>,
//     collision_sound: Res<CollisionSound>,
// ) {
//     for (mut ball_velocity, ball_transform, ball) in &mut ball_query {
//         for (other_entity, transform, other, opt_block) in &collider_query {
//             let collision = collide(
//                 ball_transform.translation,
//                 ball.size,
//                 transform.translation,
//                 other.size,
//             );

//             let mut reflect_x = false;
//             let mut reflect_y = false;
//             if let Some(collision) = collision {
//                 match collision {
//                     Collision::Left => reflect_x = ball_velocity.x > 0.0,
//                     Collision::Right => reflect_x = ball_velocity.x < 0.0,
//                     Collision::Top => reflect_y = ball_velocity.y < 0.0,
//                     Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
//                     Collision::Inside => (),
//                 }

//                 if reflect_x {
//                     ball_velocity.x *= -1.0;
//                 }

//                 if reflect_y {
//                     ball_velocity.y *= -1.0;
//                 }

//                 if opt_block.is_some() {
//                     commands.entity(other_entity).despawn();
//                     scoreboard.score += 1;
//                 }

//                 // Play sound
//                 commands.spawn(AudioBundle {
//                     source: collision_sound.to_owned(),
//                     settings: PlaybackSettings::DESPAWN,
//                 });
//             }
//         }
//     }
// }

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
