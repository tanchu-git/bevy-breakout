use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

// Paddle details
const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;

// Ball details
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BALL_STARTING_POS: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_paddle, apply_velocity))
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Spawn paddle
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(0.0, PADDLE_START_Y, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                custom_size: Some(PADDLE_SIZE),
                ..default()
            },
            ..default()
        },
        Paddle,
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
        Ball,
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

    let x_pos =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time_step.delta_seconds();

    paddle_transform.translation.x = x_pos;
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time>) {
    let delta_time = time_step.delta_seconds();

    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * delta_time;
        transform.translation.y += velocity.y * delta_time;
    }
}
