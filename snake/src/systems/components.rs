use bevy::prelude::*;

pub const PLAYER_DIMENSIONS: f32 = 40.;
pub const FOOD_DIMENSIONS: f32 = 20.;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right
}
#[derive(Component)]
pub struct Score {
    pub value: usize
}
#[derive(Component)]
pub struct SnakeHead;
#[derive(Component)]
pub struct SnakeTail;

#[derive(Component)]
pub struct Food;

#[derive(Default, Resource)]
pub struct SnakeParts(pub Vec<Entity>);
#[derive(Component)]
pub struct ScoreText;