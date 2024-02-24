// will separate it into different files in the future

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::{
    math::bounding::*, 
    prelude::*, 
    sprite::MaterialMesh2dBundle, time::common_conditions::on_timer,
};
use rand::*;
fn main() {
    let mut binding = App::new();
    let app = binding
        .add_plugins(DefaultPlugins)
        .insert_resource(SnakeParts::default())
        .init_state::<Direction>()
        .add_systems(Startup, setup)
        .add_systems(Update, (move_snake.run_if(on_timer(Duration::from_millis(100))), update_direction, eating_system, score_rendering_system, snake_length,tail_movement.run_if(on_timer(Duration::from_millis(100))), snake_collision));
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
pub struct Score {
    pub value: usize
}
#[derive(Component)]
struct SnakeHead;
#[derive(Component)]
struct SnakeTail;

#[derive(Component)]
struct Food;

#[derive(Default, Resource)]
pub struct SnakeParts(pub Vec<Entity>);
#[derive(Component)]
pub struct ScoreText;
pub const PLAYER: f32 = 40.;
pub const FOOD_DIMENSIONS: f32 = 20.;
use std::time::Duration;
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut snake_parts: ResMut<SnakeParts>,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts\\FiraSans-Bold.ttf");
    commands.spawn(Camera2dBundle::default());
    *snake_parts = SnakeParts(vec![
        commands.spawn((MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(PLAYER, PLAYER)).into(),
            transform: Transform::from_translation(Vec3::new(-96., 0., 0.)),
            material: materials.add(ColorMaterial::default()),
            ..default()
        }, SnakeHead)).insert(Score { value: 0}).id()
    ]);
    // commands.spawn((MaterialMesh2dBundle {
    //     mesh: meshes.add(Rectangle::new(PLAYER, PLAYER)).into(),
    //     transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
    //     material: materials.add(ColorMaterial::default()),
    //     ..default()
    // }, SnakeTail));
        commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(ScoreText);
    spawn_apple(commands)
}
fn move_snake(time: Res<Time>, mut snake: Query<&mut Transform, With<SnakeHead>>,direction: Res<State<Direction>>,) {
    for mut transform in &mut snake {
        match **direction {
            Direction::Up => transform.translation.y += PLAYER ,
            Direction::Down => transform.translation.y -= PLAYER ,
            Direction::Left => transform.translation.x -= PLAYER ,
            Direction::Right => transform.translation.x += PLAYER ,
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
fn eating_system(
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
         if ((((food.translation.x - head_position.x) as f32).powf(2.0) + ((food.translation.y - head_position.y) as f32).powf(2.0)) as f32).sqrt() < (PLAYER / 2.0 + FOOD_DIMENSIONS / 2.0) {
            commands.get_entity(entity).unwrap().despawn();
            scored = 1;
    }}
    if scored == 1 {
            set.p0().get_single_mut().unwrap().0.value += 1;
            println!("SNAKE ATE {}", set.p0().get_single().unwrap().0.value );
            spawn_apple(commands);
    }
}
fn score_rendering_system(mut text_query: Query<&mut Text, With<ScoreText>>, mut score_query: Query<&Score, With<SnakeHead>>) {
    for mut text in text_query.iter_mut() {
        let score = score_query.iter_mut().next().unwrap().value;
        text.sections[1].value = format!("{:.2}", score);
    }
}
fn spawn_tail(commands: &mut Commands) -> Entity {
commands.spawn(SpriteBundle {
          sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PLAYER, PLAYER)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            0.,
            0.,
        10.
        )), 
        ..default()
}).insert(SnakeTail).id()
}
fn snake_length(mut commands: Commands, mut snake_parts: ResMut<SnakeParts>, mut snake_head: Query<(&mut Score ,&mut Transform), With<SnakeHead>>) {
    for (score, _head) in snake_head.iter_mut() {
        if (score.value + 1) > snake_parts.0.len() {
            snake_parts.0.push(spawn_tail(&mut commands))
        }
    }
}
fn tail_movement(mut set: ParamSet<(Query<&mut Transform>, Query<&mut Transform, With<SnakeTail>>)>, snake_parts: ResMut<SnakeParts>) {
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
        let prev_value = *part;
        if i != 0 {
            part.translation.y = prev.translation.y;
            part.translation.x = prev.translation.x;

        }
        prev = prev_value;
        }
    }

}
fn snake_collision(
    mut set: ParamSet<(
        Query<(&mut Score ,&mut Transform, Entity), With<SnakeHead>>,
        Query<(Entity, &mut Transform), With<SnakeTail>>,
    )>, 
    snake_parts: ResMut<SnakeParts>,
    mut commands: Commands
) {
    let head_position = {
        let p0 = set.p0();
        let head = p0.get_single().unwrap().1;
        Vec2::new(head.translation.x, head.translation.y)
    }; 
    let mut lost = false;
    for (_e, tail) in set.p1().iter() {
    if ((((tail.translation.x - head_position.x) as f32).powf(2.0) + ((tail.translation.y - head_position.y) as f32).powf(2.0)) as f32).sqrt() < (PLAYER / 2.0 + PLAYER / 2.0) {
        lost = true
    }}
    if lost == true {
        restart(&mut commands, set, snake_parts)
    }
}
fn restart(commands: &mut Commands,mut set: ParamSet<(
        Query<(&mut Score ,&mut Transform, Entity), With<SnakeHead>>,
        Query<(Entity, &mut Transform), With<SnakeTail>>,
    )>, mut snake_parts: ResMut<SnakeParts>) {
        snake_parts.0.clear();
        snake_parts.0.push(set.p0().get_single().unwrap().2);
    for tail in set.p1().iter_mut() {
        commands.get_entity(tail.0).unwrap().despawn();
    }
    set.p0().get_single_mut().unwrap().0.value = 0;
    set.p0().get_single_mut().unwrap().1.translation.x = 0.;
    set.p0().get_single_mut().unwrap().1.translation.y = 0.;
}