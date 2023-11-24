use bevy::{math::vec2, prelude::*};

use crate::{
    resources::Collider,
    walls::{BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, TOP_WALL},
};

// Blocks
const BLOCK_SIZE: Vec2 = Vec2::new(100.0, 30.0);
const BLOCK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const GAP_BETWEEN_PADDLE_AND_BLOCKS: f32 = 270.0;
const GAP_BETWEEN_BLOCKS: f32 = 5.0;
const GAP_BETWEEN_BLOCKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BLOCKS_AND_SIDES: f32 = 20.0;

#[derive(Component)]
pub(crate) struct Block;

impl Block {
    pub(crate) fn spawn_blocks(commands: &mut Commands) {
        let offset_x = LEFT_WALL + GAP_BETWEEN_BLOCKS_AND_SIDES + BLOCK_SIZE.x * 0.5;
        let offset_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_BLOCKS + BLOCK_SIZE.y * 0.5;

        let blocks_total_width = (RIGHT_WALL - LEFT_WALL) - 2.0 * GAP_BETWEEN_BLOCKS_AND_SIDES;
        let blocks_total_height = (TOP_WALL - BOTTOM_WALL)
            - GAP_BETWEEN_BLOCKS_AND_CEILING
            - GAP_BETWEEN_PADDLE_AND_BLOCKS;

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
    }
}
