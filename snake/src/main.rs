// will separate it into different files in the future

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::{
    math::bounding::*, 
    prelude::*, 
    sprite::MaterialMesh2dBundle
};
use rand::*;
fn main() {
    let mut binding = App::new();
    let app = binding
        .add_plugins(DefaultPlugins)
        .insert_resource(SnakeParts::default())
        .init_state::<Direction>()
        .add_systems(Startup, setup)
        .add_systems(Update, (move_snake, update_direction, eating_system, score_rendering_system, snake_length, tail_movement));
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
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tail_parts: ResMut<SnakeParts>,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts\\FiraSans-Bold.ttf");
    commands.spawn(Camera2dBundle::default());
    *tail_parts = SnakeParts(vec![
        commands.spawn((MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(PLAYER, PLAYER)).into(),
            transform: Transform::from_translation(Vec3::new(-96., 0., 0.)),
            material: materials.add(ColorMaterial::default()),
            ..default()
        }, SnakeHead)).insert(Score { value: 0}).id(),
        spawn_tail(&mut commands)
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
            Direction::Up => transform.translation.y += PLAYER * time.delta_seconds(),
            Direction::Down => transform.translation.y -= PLAYER * time.delta_seconds(),
            Direction::Left => transform.translation.x -= PLAYER * time.delta_seconds(),
            Direction::Right => transform.translation.x += PLAYER * time.delta_seconds(),
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
}).id()
}
fn snake_length(mut commands: Commands, mut snake_parts: ResMut<SnakeParts>, mut snake_head: Query<(&mut Score ,&mut Transform), With<SnakeHead>>) {
    for (score, _head) in snake_head.iter_mut() {
        if (score.value + 1) > snake_parts.0.len() {
            println!("NOOOOOOOO");
            snake_parts.0.push(spawn_tail(&mut commands))
        }
    }
}
fn tail_movement(mut transform_query: Query<&mut Transform>, snake_parts: ResMut<SnakeParts>) {
    let head = *transform_query.get(snake_parts.0[0]).unwrap();
    for (i, p) in snake_parts.0.iter().enumerate() {
        println!("{:?}", p);
        // let mut part = transform_query.get_mut(*p).unwrap();
        // if i != 0 {
        //     part.translation.y = head.translation.y - PLAYER;
        //     part.translation.x = head.translation.x - PLAYER
        // }
        // println!("{:?}, {i}", part);
    }
}