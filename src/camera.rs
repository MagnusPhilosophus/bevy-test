use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use std::f32::consts::PI;

#[derive(Component)]
pub struct FlyCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-5.0, 5.0, -5.0)
                .looking_at(Vec3::new(0.0, -2.0, 0.0), Vec3::Y),
            ..default()
        },
        FlyCamera,
    ));
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 20.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.0),
            ..default()
        },
        ..default()
    });
}

fn camera_move(
    mut transform: Query<&mut Transform, With<FlyCamera>>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    let mut transform = transform
        .get_single_mut()
        .expect("Only one camera with FlyCamera");
    let mut velocity = Vec3::ZERO;
    for key in keys.get_pressed() {
        match key {
            KeyCode::Comma => velocity += transform.forward(),
            KeyCode::O => velocity -= transform.forward(),
            KeyCode::E => velocity += transform.right(),
            KeyCode::A => velocity -= transform.right(),
            KeyCode::Space => velocity += Vec3::Y,
            KeyCode::ShiftLeft => velocity -= Vec3::Y,
            _ => (),
        }
    }
    transform.translation += velocity.normalize_or_zero() * time.delta_seconds() * 2.0;
}

fn camera_look(
    mut transform: Query<&mut Transform, With<FlyCamera>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let window = window.get_single().expect("Failed to find a window");
    let mut transform = transform
        .get_single_mut()
        .expect("Only one camera with FlyCamera");

    for event in mouse_motion.iter() {
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        let window_scale = window.height().min(window.width());
        pitch -= (0.0001 * event.delta.y * window_scale).to_radians();
        yaw -= (0.0001 * event.delta.x * window_scale).to_radians();
        pitch = pitch.clamp(-1.54, 1.54);
        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    }
}

fn toggle_grab_cursor(window: &mut Window) {
    if window.cursor.grab_mode == CursorGrabMode::None {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    } else {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

fn grab_cursor(mut window: Query<&mut Window, With<PrimaryWindow>>, keys: Res<Input<KeyCode>>) {
    let mut window = window.get_single_mut().expect("Failed to find window");
    if keys.just_pressed(KeyCode::Tab) {
        toggle_grab_cursor(&mut window);
    }
}

fn initial_grab_cursor(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window.get_single_mut().expect("Failed to find window");
    toggle_grab_cursor(&mut window);
}

pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, initial_grab_cursor))
            .add_systems(Update, (camera_move, camera_look, grab_cursor));
    }
}
