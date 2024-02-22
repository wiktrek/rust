//! Shows how to render simple primitive shapes with a single color.
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::{
    math::bounding::*, 
    prelude::*, 
    sprite::MaterialMesh2dBundle
};

fn main() {
    let mut binding = App::new();
    let app = binding
        .add_plugins(DefaultPlugins)
        .init_state::<Direction>()
        .add_systems(Startup, setup)
        .add_systems(Update, (move_snake, update_direction, check_if_lost_or_ate));
    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());
    app.run()
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
struct SnakeHead;
#[derive(Component)]
struct SnakeTail;

#[derive(Component)]
struct Food;
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(30.0, 30.0)).into(),
        transform: Transform::from_translation(Vec3::new(-96., 0., 0.)),
        material: materials.add(ColorMaterial::default()),
        ..default()
    }, SnakeHead));
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(30.0, 30.0)).into(),
        transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
        material: materials.add(ColorMaterial::default()),
        ..default()
    }, SnakeTail));
    spawn_apple(commands, meshes, materials)
}
fn move_snake(time: Res<Time>, mut snake: Query<&mut Transform, With<SnakeHead>>,direction: Res<State<Direction>>,) {
    for mut transform in &mut snake {
    //     match **direction {
    //         Direction::Up => transform.translation.y += 50. * time.delta_seconds(),
    //         Direction::Down => transform.translation.y -= 50. * time.delta_seconds(),
    //         Direction::Left => transform.translation.x -= 50. * time.delta_seconds(),
    //         Direction::Right => transform.translation.x += 50. * time.delta_seconds(),
    // }
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
fn check_if_lost_or_ate(
    mut set: ParamSet<(
        Query<&mut Transform, With<SnakeHead>>,
        Query<&mut Transform, With<Food>>,
        Query<&mut Transform, With<SnakeTail>>,
    )>,
) {
    let head_position = {
        let p0 = set.p0();
        let head = p0.get_single().unwrap();
        Vec2::new(head.translation.x, head.translation.y)
    };

    let tail_positions: Vec<Vec2> = set
        .p2()
        .iter_mut()
        .map(|t| Vec2::new(t.translation.x, t.translation.y))
        .collect();

    for &tail_position in &tail_positions {
        if head_position.x == tail_position.x && head_position.y == tail_position.y {
            println!("Game Over: Snake collided with itself!");
            // Handle game over logic here
            return;
        }
    }

    // Separate block for the second mutable borrow
    {
        let p1 = set.p1();
        let food = p1.get_single().unwrap();
        if head_position.x == food.translation.x && head_position.y == food.translation.y {
            println!("Snake ate the food!");
            // Handle food eating logic here
        }
    }
}