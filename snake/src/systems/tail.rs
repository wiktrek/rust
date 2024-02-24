use bevy::prelude::*;

use crate::systems::components::{SnakeParts, SnakeHead,SnakeTail, PLAYER_DIMENSIONS, Score};
pub fn snake_length(mut commands: Commands, mut snake_parts: ResMut<SnakeParts>, mut snake_head: Query<(&mut Score ,&mut Transform), With<SnakeHead>>) {
    for (score, _head) in snake_head.iter_mut() {
        if (score.value + 1) > snake_parts.0.len() {
            snake_parts.0.push(spawn_tail(&mut commands))
        }
    }
}
pub fn tail_movement(mut set: ParamSet<(Query<&mut Transform>, Query<&mut Transform, With<SnakeTail>>)>, snake_parts: ResMut<SnakeParts>) {
    let len = snake_parts.0.len();
    let mut check= false;
    if len == set.p1().iter().len() + 1 {
        check = true;
    }
    if check == true {
        let mut prev = *set.p0().get(snake_parts.0[0]).unwrap();
        for (i, p) in snake_parts.0.iter().enumerate().skip(1) {
        let mut p0 = set.p0();
        let mut part = p0.get_mut(*p).unwrap();
        println!("{} {:?} {:?}", i, prev.translation, part.translation);
        let prev_value = *part;
            part.translation.y = prev.translation.y;
            part.translation.x = prev.translation.x;
            prev = prev_value;
        }
    }

}
pub fn spawn_tail(commands: &mut Commands) -> Entity {
commands.spawn(SpriteBundle {
          sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PLAYER_DIMENSIONS - 10., PLAYER_DIMENSIONS - 10.)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            0.,
            0.,
        0.
        )), 
        ..default()
}).insert(SnakeTail).id()
}
