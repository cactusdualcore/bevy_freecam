//! Adapted from the [basic 3d bevy example](https://bevyengine.org/examples/3d-rendering/3d-scene/).
//! This example includes the debug camera and shows its exemplatory usage.

use bevy::prelude::*;
use bevy_awesome_debug_camera::{DebugCamera, DebugCameraPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // The plugin contains all movement related functionality, so make sure to add it if you desire this.
            DebugCameraPlugin::new_with_keybindings(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // The `DebugCamera` component marks a camera as eglible to be taken over by this plugin.
        DebugCamera::default(),
    ));
}
