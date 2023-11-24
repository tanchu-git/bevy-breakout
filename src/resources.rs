use bevy::prelude::*;

// Scoreboard
const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Component, Deref, DerefMut)]
pub(crate) struct Velocity(pub(crate) Vec2);

#[derive(Component)]
pub(crate) struct Collider {
    pub(crate) size: Vec2,
}

#[derive(Resource)]
pub(crate) struct Scoreboard {
    pub(crate) score: usize,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct CollisionSound(pub(crate) Handle<AudioSource>);

impl Scoreboard {
    pub fn spawn_scoreboard(commands: &mut Commands) {
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

    pub fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
        let mut text = query.single_mut();
        text.sections[1].value = scoreboard.score.to_string();
    }
}

impl Velocity {
    pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time>) {
        let delta_time = time_step.delta_seconds();

        for (mut transform, velocity) in &mut query {
            transform.translation.x += velocity.x * delta_time;
            transform.translation.y += velocity.y * delta_time;
        }
    }
}

impl CollisionSound {
    pub(crate) fn load_sound_files(commands: &mut Commands, asset_server: &AssetServer) {
        let collision_sound = asset_server.load("sounds/breakout_collision.ogg");
        commands.insert_resource(CollisionSound(collision_sound));
    }
}
