use bevy::prelude::*;

use crate::systems::components::{Direction, SnakeHead, PLAYER_DIMENSIONS};
pub fn move_snake(mut snake: Query<&mut Transform, With<SnakeHead>>,direction: Res<State<Direction>>,) {
    for mut transform in &mut snake {
        match **direction {
            Direction::Up => transform.translation.y += PLAYER_DIMENSIONS ,
            Direction::Down => transform.translation.y -= PLAYER_DIMENSIONS ,
            Direction::Left => transform.translation.x -= PLAYER_DIMENSIONS ,
            Direction::Right => transform.translation.x += PLAYER_DIMENSIONS ,
    }
}
}