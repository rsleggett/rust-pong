use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{bundles::GutterBundle, constants::GUTTER_HEIGHT};

pub fn spawn_gutters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_height = window.resolution.height();
        let window_width = window.resolution.width();

        let top_gutter_y = window_height / 2. - GUTTER_HEIGHT / 2.;
        let bottom_gutter_y = -window_height / 2. + GUTTER_HEIGHT / 2.;

        let top_gutter = GutterBundle::new(0., top_gutter_y, window_width);
        let bottom_gutter = GutterBundle::new(0., bottom_gutter_y, window_width);

        let shape = Rectangle::from_size(top_gutter.shape.0);
        let color = Color::srgb(0., 0., 0.);

        let mesh = meshes.add(shape);
        let material = materials.add(color);

        commands.spawn((
            top_gutter,
            MaterialMesh2dBundle {
                mesh: mesh.clone().into(),
                material: material.clone(),
                ..default()
            }
        ));

        commands.spawn((
            bottom_gutter,
            MaterialMesh2dBundle {
                mesh: mesh.into(),
                material: material,
                ..default()
            }
        ));
    }
}