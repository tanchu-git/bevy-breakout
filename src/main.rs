use bevy::{
    math::{vec2, vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

// Paddle details
const PADDLE_START_Y: f32 = -240.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const _PADDLE_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
const PADDLE_SPEED: f32 = 500.0;

// Ball details
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BALL_STARTING_POS: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const BALL_SPEED: f32 = 400.0;
const BALL_INIT_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

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
                move_paddle,
                apply_velocity,
                ball_collision.after(apply_velocity),
            ),
        )
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball {
    size: Vec2,
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

    // Load texture and spawn paddle
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

    // Load texture and spawn ball
    let ball_texture = asset_server.load("textures/ball.png");

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
        Velocity(BALL_SPEED * BALL_INIT_DIRECTION),
    ));

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

fn move_paddle(
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

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time>) {
    let delta_time = time_step.delta_seconds();

    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * delta_time;
        transform.translation.y += velocity.y * delta_time;
    }
}

fn ball_collision(
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
            }
        }
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
