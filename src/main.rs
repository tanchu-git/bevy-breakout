use ball::Ball;
use bevy::prelude::*;
use blocks::Block;
use paddle::Paddle;
use resources::{CollisionSound, Scoreboard, Velocity};
use walls::WallBundle;

mod ball;
mod blocks;
mod paddle;
mod resources;
mod walls;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Scoreboard { score: 0 })
        .add_systems(
            Update,
            (bevy::window::close_on_esc, Scoreboard::update_scoreboard),
        )
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                Velocity::apply_velocity,
                Paddle::move_paddle,
                Ball::ball_collision,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Load sound resources
    CollisionSound::load_sound_files(&mut commands, &asset_server);

    // Spawn components
    Paddle::spawn_paddle(&mut commands, &asset_server);

    Ball::spawn_ball(&mut commands, &asset_server);

    WallBundle::spawn_walls(&mut commands);

    Block::spawn_blocks(&mut commands);

    Scoreboard::spawn_scoreboard(&mut commands);
}
