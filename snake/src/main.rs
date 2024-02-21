//! Shows how to render simple primitive shapes with a single color.

use bevy::{
    prelude::*,  sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

const X_EXTENT: f32 = 300.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let shapes = vec![
        Mesh2dHandle(meshes.add(Rectangle::new(30.0,30.0)).into()); 100
    ];
    let mut x = 0.0;
    let mut y = 0.0;
    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(0., 7., 50.);

        commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT to +X_EXTENT.
                -X_EXTENT / 2. + x as f32 / (10 ) as f32 * X_EXTENT,
                y,
                0.0,
            ),
            ..default()
        });
        if x != 9.0 {
            x += 1.0
        } else {
            x = 0.0;
            y += 30.0;
        }
    }
}