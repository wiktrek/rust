use bevy::prelude::*; 
use rand::*;
use crate::systems::components::{FOOD_DIMENSIONS, Food, SnakeHead, Score, PLAYER_DIMENSIONS};
pub fn spawn_apple(
    mut commands: Commands,
) {
commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(FOOD_DIMENSIONS, FOOD_DIMENSIONS)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            rand::thread_rng().gen_range(1..500) as f32,
            rand::thread_rng().gen_range(1..300) as f32,
        10.
        )), 
        ..default()
}, Food));
}
pub fn eating_system(
    mut set: ParamSet<(
        Query<(&mut Score ,&mut Transform), With<SnakeHead>>,
        Query<(Entity, &mut Transform), With<Food>>,
    )>, mut commands: Commands,
) {
    let head_position = {
        let p0 = set.p0();
        let head = p0.get_single().unwrap().1;
        Vec2::new(head.translation.x, head.translation.y)
    };
    let mut scored = 0;
    for (entity, food) in set.p1().iter_mut() {
         if ((((food.translation.x - head_position.x) as f32).powf(2.0) + ((food.translation.y - head_position.y) as f32).powf(2.0)) as f32).sqrt() < (PLAYER_DIMENSIONS / 2.0 + FOOD_DIMENSIONS / 2.0) {
            commands.get_entity(entity).unwrap().despawn();
            scored = 1;
    }}
    if scored == 1 {
            set.p0().get_single_mut().unwrap().0.value += 1;
            println!("SNAKE ATE {}", set.p0().get_single().unwrap().0.value );
            spawn_apple(commands);
    }
}