use bevy::prelude::*; 
use crate::systems::components::Direction;
pub fn update_direction(mut direction: ResMut<NextState<Direction>>, keyboard: Res<ButtonInput<KeyCode>>,) {
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction.set(Direction::Right)
    }
      if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction.set(Direction::Left)
    }   if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        direction.set(Direction::Up)
    }   if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS)  {
        direction.set(Direction::Down)
    }
}