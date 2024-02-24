// will separate it into different files in the future

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::{
    prelude::*, 
    sprite::MaterialMesh2dBundle, time::common_conditions::on_timer,
};
use std::time::Duration;
use rand::*;
mod systems;
use systems::*;
fn main() {
    let mut binding = App::new();
    let app = binding
        .add_plugins(DefaultPlugins)
        .insert_resource(SnakeParts::default())
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .init_state::<Direction>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            move_snake.run_if(on_timer(Duration::from_millis(100))),
             update_direction, 
             eating_system, 
             score_rendering_system, 
             snake_length,
             tail_movement.run_if(on_timer(Duration::from_millis(100))), 
             snake_collision
            ));
    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());
    app.run()
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
        0.
        )), 
        ..default()
}).insert(SnakeTail).id()
}
