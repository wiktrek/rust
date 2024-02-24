use bevy::{
    prelude::*, 
    sprite::MaterialMesh2dBundle
};

use crate::*;
pub fn setup(
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
            mesh: meshes.add(Rectangle::new(PLAYER_DIMENSIONS, PLAYER_DIMENSIONS)).into(),
            transform: Transform::from_translation(Vec3::new(-96., 0., 0.)),
            material: materials.add(ColorMaterial::default()),
            ..default()
        }, SnakeHead)).insert(Score { value: 0}).id()
    ]);
    commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(510. * 2.0, 310. * 2.0)).into(),
            transform: Transform::from_translation(Vec3::new(0., 0., -10.)),
            material: materials.add(Color::GREEN),
            ..default()
        });
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