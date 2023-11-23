use ball::Ball;
use bevy::{
    math::{vec2, vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use paddle::Paddle;
use walls::{WallBundle, BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, TOP_WALL};

mod ball;
mod paddle;
mod walls;

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

// #[derive(Bundle)]
// struct WallBundle {
//     sprite_bundle: SpriteBundle,
//     collider: Collider,
// }

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

    Ball::spawn_ball(&mut commands, &asset_server);

    // Spawn walls
    WallBundle::spawn_walls(&mut commands);

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

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
