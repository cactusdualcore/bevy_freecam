use std::f32::consts::{FRAC_PI_4, TAU};
use std::ops::RangeInclusive;

use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::{PrimaryWindow, WindowRef};

#[derive(Debug, Reflect, Clone)]
pub struct DebugCamera {
    enabled: bool,
    anchor: Option<Vec3>,
    origin: Option<Transform>,
}

impl Default for DebugCamera {
    fn default() -> Self {
        Self {
            enabled: true,
            anchor: None,
            origin: None,
        }
    }
}

impl Component for DebugCamera {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            let options = world.resource::<DebugCameraOptions>();

            if options.remember_original_transform {
                let transform = world
                    .get::<Transform>(entity)
                    .cloned()
                    .expect("added 'DebugCamera' to entity without 'Transform'");
                let mut debug_camera = world.get_mut::<Self>(entity).unwrap();
                let _ = debug_camera.origin.insert(transform);
            }
        });
    }
}

#[derive(Debug, Reflect, Clone, Resource)]
pub struct DebugCameraOptions {
    /// Whether the debug camera is enabled globally. Particular Cameras can
    /// still be disabled individually if this is enabled, but the opposite
    /// is not true. Defaults to `false`.
    pub enabled: bool,
    /// Whether `DebugCamera`s should remember their initial `Transform` when
    /// added or enabled. Defaults to `true`.
    pub remember_original_transform: bool,
    /// The movement speed of a camera in meters per second.
    /// Defaults to `2.0` m/s.
    pub movement_speed: f32,
    /// The fast movement speed a camera in meters per second.
    /// Defaults to `3.0` m/s.
    pub fast_movement_speed: f32,
    /// The angle to be rotated on view direction change in radians per second.
    /// The intensity of mouse movements is adjustable in both the horizontal
    /// and vertical direction through this value. Defaults to 6° in
    /// both directions, or `Vec2::splat(-TAU / 60.0)`.
    pub turning_speed: Vec2,
    /// Defaults to `2.0`.
    pub zoom_speed: f32,
    /// Defaults to `0.1..=100.0`.
    pub zoom_distance_range: RangeInclusive<f32>,
    pub input_options: InputOptions,
    /// The range of angles in radians wherein the camera is allowed freely
    /// rotate up and down. Rotating outside this range will clamp the looking
    /// direction back into it. Zero is in the direction of
    /// `Transform::forward`. Providing `None` disables clamping completely.
    /// Defaults to 45° up and down, or `Some(-FRAC_PI_4..=FRAC_PI_4)`
    pub vertical_fov: Option<RangeInclusive<f32>>,
}

impl Default for DebugCameraOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            remember_original_transform: true,
            movement_speed: 2.0,
            fast_movement_speed: 3.0,
            turning_speed: Vec2::splat(-TAU / 60.0),
            zoom_speed: 2.0,
            zoom_distance_range: 0.1..=100.0,
            input_options: InputOptions::default(),
            vertical_fov: Some(-FRAC_PI_4..=FRAC_PI_4),
        }
    }
}

impl DebugCameraOptions {
    pub fn default_with_keybindings() -> Self {
        Self {
            input_options: InputOptions {
                keybindings: KeyBindings::default(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

#[derive(Debug, Reflect, Clone)]
#[non_exhaustive]
pub struct InputOptions {
    pub keybindings: KeyBindings,
}

impl Default for InputOptions {
    fn default() -> Self {
        Self {
            keybindings: KeyBindings::EMPTY,
        }
    }
}

#[derive(Debug, Reflect, Clone)]
pub struct KeyBindings {
    pub forward: Option<KeyCode>,
    pub back: Option<KeyCode>,
    pub left: Option<KeyCode>,
    pub right: Option<KeyCode>,
    pub up: Option<KeyCode>,
    pub down: Option<KeyCode>,
    pub global_up: Option<KeyCode>,
    pub global_down: Option<KeyCode>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl KeyBindings {
    pub const EMPTY: Self = Self {
        forward: None,
        back: None,
        left: None,
        right: None,
        up: None,
        down: None,
        global_up: None,
        global_down: None,
    };

    pub const DEFAULT: Self = Self {
        forward: Some(KeyCode::KeyW),
        back: Some(KeyCode::KeyS),
        left: Some(KeyCode::KeyA),
        right: Some(KeyCode::KeyD),
        up: Some(KeyCode::KeyQ),
        down: Some(KeyCode::KeyE),
        global_up: Some(KeyCode::KeyR),
        global_down: Some(KeyCode::KeyF),
    };
}

fn debug_camera_is_globally_enabled(o: Res<DebugCameraOptions>) -> bool {
    o.enabled
}

#[derive(Debug, Default)]
pub struct DebugCameraPlugin {
    debug_camera_options: DebugCameraOptions,
}

impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.debug_camera_options.clone())
            .add_systems(
                Update,
                (
                    move_mouse_to_rotate,
                    mouse_scroll_to_move_radially_or_zoom,
                    keyboard_input_to_movements,
                )
                    .run_if(debug_camera_is_globally_enabled),
            )
            .add_systems(
                PostUpdate,
                (
                    clamp_camera_rotation_vertically
                        .run_if(|options: Res<DebugCameraOptions>| options.vertical_fov.is_some()),
                    force_camera_up_in_y_forward_plane,
                )
                    .run_if(debug_camera_is_globally_enabled),
            );
    }
}

impl DebugCameraPlugin {
    pub fn new_with_keybindings() -> Self {
        Self {
            debug_camera_options: DebugCameraOptions::default_with_keybindings(),
        }
    }

    pub fn enable_by_default(mut self) -> Self {
        self.debug_camera_options.enabled = true;
        return self;
    }
}

fn move_mouse_to_rotate(
    mut mouse_motion: EventReader<MouseMotion>,
    debug_camera_options: Res<DebugCameraOptions>,
    mut debug_cameras: Query<(&DebugCamera, &mut Transform)>,
    time: Res<Time>,
) {
    let radians_to_turn = debug_camera_options.turning_speed * time.delta_seconds();
    let rotational_delta = radians_to_turn * mouse_motion.read().map(|m| m.delta).sum::<Vec2>();

    mouse_motion.clear();

    for (_, mut transform) in debug_cameras.iter_mut() {
        transform.rotate_y(rotational_delta.x);
        transform.rotate_local_x(rotational_delta.y);
    }
}

fn clamp_camera_rotation_vertically(
    debug_camera_options: Res<DebugCameraOptions>,
    mut debug_cameras: Query<&mut Transform, (With<DebugCamera>, With<Camera>)>,
) {
    let vertical_fov = debug_camera_options.vertical_fov.as_ref().unwrap();

    for mut transform in debug_cameras.iter_mut() {
        assert_ne!(*transform.forward(), Vec3::Y);

        let forward = transform.forward();

        let flat_forward = forward.with_y(0.0).normalize();
        // The normal of the plane spanned by Vec3::Y and flat_forward.
        // The call to 'normalize' is redundant, but included for clarity.
        let n = flat_forward.cross(Vec3::Y).normalize();

        // The right-handed signed rotation angle is chosen for correct behaviour with rotations around 'Transform::left'.
        // adapted from https://stackoverflow.com/questions/5188561/signed-angle-between-two-3d-vectors-with-same-origin-within-the-same-plane
        let theta_righthanded = f32::atan2(
            forward.cross(flat_forward).dot(n),
            forward.dot(flat_forward),
        );

        if !vertical_fov.contains(&theta_righthanded) {
            let theta_in_fov = theta_righthanded.clamp(*vertical_fov.start(), *vertical_fov.end());
            let rotation = Quat::from_axis_angle(*transform.left(), theta_in_fov);
            transform.look_to(rotation * flat_forward, Vec3::Y);
        }
    }
}

fn force_camera_up_in_y_forward_plane(
    mut debug_cameras: Query<&mut Transform, (With<DebugCamera>, With<Camera>)>,
) {
    for mut transform in debug_cameras.iter_mut() {
        let y_forward_plane_normal = transform.forward().cross(Vec3::Y).normalize();

        let projection = transform
            .up()
            .project_onto_normalized(y_forward_plane_normal);

        let rotation = Quat::from_rotation_arc(*transform.up(), *transform.up() - projection);
        transform.rotate(rotation);
    }
}

fn mouse_scroll_to_move_radially_or_zoom(
    mut mouse_wheel_movements: EventReader<MouseWheel>,
    debug_camera_options: Res<DebugCameraOptions>,
    mut debug_cameras: Query<(&Camera, &mut Projection), With<DebugCamera>>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let primary_window_entity = primary_window_query
        .get_single()
        .expect("there should only be exactly one primary window");
    for (camera, mut projection) in debug_cameras.iter_mut() {
        if let RenderTarget::Window(window) = camera.target {
            let window_entity = match window {
                WindowRef::Primary => primary_window_entity,
                WindowRef::Entity(entity) => entity,
            };

            let pixels_scrolled = mouse_wheel_movements
                .read()
                .filter(|mw| mw.window == window_entity)
                .map(|mw| mw.y)
                .sum::<f32>();

            let dr = pixels_scrolled * debug_camera_options.zoom_speed * time.delta_seconds();

            let minimum_zoom = *debug_camera_options.zoom_distance_range.start();
            let maximum_zoom = *debug_camera_options.zoom_distance_range.end();
            match projection.as_mut() {
                Projection::Perspective(projection) => {
                    projection.near = (projection.near + dr).clamp(minimum_zoom, maximum_zoom);
                }
                Projection::Orthographic(projection) => {
                    projection.near = (projection.near + dr).clamp(minimum_zoom, maximum_zoom);
                }
            };
        }
    }
}

fn keyboard_input_to_movements(
    keys: Res<ButtonInput<KeyCode>>,
    debug_camera_options: Res<DebugCameraOptions>,
    mut debug_cameras: Query<(&DebugCamera, &mut Transform)>,
    time: Res<Time>,
) {
    for (_, mut transform) in debug_cameras.iter_mut() {
        let keybindings = &debug_camera_options.input_options.keybindings;

        let directions = [
            (keybindings.forward, transform.forward()),
            (keybindings.left, transform.left()),
            (keybindings.back, transform.back()),
            (keybindings.right, transform.right()),
            (keybindings.up, transform.up()),
            (keybindings.down, transform.down()),
            (keybindings.global_up, Dir3::Y),
            (keybindings.global_down, Dir3::NEG_Y),
        ];

        let delta = directions
            .into_iter()
            .filter(|(keybinding, _)| keybinding.is_some_and(|key_code| keys.pressed(key_code)))
            .map(|(_, direction)| {
                direction * debug_camera_options.movement_speed * time.delta_seconds()
            })
            .sum::<Vec3>();

        transform.translation += delta;
    }
}
