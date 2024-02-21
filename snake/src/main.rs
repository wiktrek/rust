//! Shows how to render simple primitive shapes with a single color.

use std::borrow::BorrowMut;

use bevy::{
    math::bounding::*, 
    prelude::*, 
    sprite::MaterialMesh2dBundle
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<Direction>()
        .add_systems(Startup, setup)
        .add_systems(Update, (move_snake, update_direction))
        .run();
}
#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right
}
#[derive(Component)]
struct Snake;

#[derive(Component)]
struct Food;
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(10.0, 10.0)).into(),
        transform: Transform::from_translation(Vec3::new(-96., 0., 0.)),
        material: materials.add(ColorMaterial::default()),
        ..default()
    }, Snake));
    spawn_apple(commands, meshes, materials)
}
fn move_snake(time: Res<Time>, mut snake: Query<&mut Transform, With<Snake>>,direction: Res<State<Direction>>,) {
    for mut transform in &mut snake {
        match **direction {
            Direction::Up => transform.translation.y += 50. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 50. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 50. * time.delta_seconds(),
            Direction::Right => transform.translation.x += 50. * time.delta_seconds(),
    }
}
}
fn update_direction(mut direction: ResMut<NextState<Direction>>, keyboard: Res<ButtonInput<KeyCode>>,) {
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction.set(Direction::Right)
    }
      if keyboard.pressed(KeyCode::ArrowLeft) {
        direction.set(Direction::Left)
    }   if keyboard.pressed(KeyCode::ArrowUp) {
        direction.set(Direction::Up)
    }   if keyboard.pressed(KeyCode::ArrowDown) {
        direction.set(Direction::Down)
    }
}
fn spawn_apple(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(
        (MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(10.0, 10.0)).into(),
        transform:  Transform::from_xyz(
                10.0,
                0.0,
                0.0,
            ),
        material: materials.add(Color::RED),
        ..default()
    },
    Food
));
    
}