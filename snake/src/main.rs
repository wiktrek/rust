// will separate it into different files in the future

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::{
    prelude::*, time::common_conditions::on_timer,
};
use std::time::Duration;

mod systems;
use systems::*;
use systems::components::Direction;
fn main() {
    let mut binding = App::new();
    let app = binding
        .add_plugins(DefaultPlugins)
        .insert_resource(SnakeParts::default())
        .insert_resource(ClearColor(Color::DARK_GREEN))
        .init_state::<Direction>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            move_snake.run_if(on_timer(Duration::from_millis(100))).after(tail_movement),
             update_direction, 
             eating_system, 
             score_rendering_system, 
             snake_length,
             tail_movement.run_if(on_timer(Duration::from_millis(100))).before(move_snake), 
             snake_collision,
            ));
    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());
    app.run()
}
