use crate::ui::MazeTimer;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_rapier3d::prelude::*;
use std::time::Instant;

#[derive(PartialEq)]
enum CameraType {
    Fly,
    Player,
}

#[derive(Component)]
pub struct CameraSettings {
    camera_type: CameraType,
    speed: f32,
    sensitivity: f32,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.0, -2.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        CameraSettings {
            camera_type: CameraType::Player,
            speed: 2.0,
            sensitivity: 0.0001,
        },
    ));
}

fn camera_move(
    mut cam_query: Query<(&mut Transform, &CameraSettings)>,
    mut player_query: Query<
        (&Transform, &mut KinematicCharacterController),
        Without<CameraSettings>,
    >,
    mut maze_timer: ResMut<MazeTimer>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    let (mut cam_t, settings) = cam_query.single_mut();
    let mut velocity = Vec3::ZERO;
    let mut speed_multiplier = 1.0;
    let forward = Vec3::new(cam_t.forward().x, 0.0, cam_t.forward().z).normalize_or_zero();
    if settings.camera_type == CameraType::Fly {
        for key in keys.get_pressed() {
            match key {
                KeyCode::Comma => velocity += forward,
                KeyCode::O => velocity -= forward,
                KeyCode::E => velocity += cam_t.right(),
                KeyCode::A => velocity += cam_t.left(),
                KeyCode::Space => velocity += Vec3::Y,
                KeyCode::ShiftLeft => velocity -= Vec3::Y,
                KeyCode::ControlLeft => speed_multiplier = 10.0,
                _ => (),
            }
        }
        cam_t.translation +=
            velocity.normalize_or_zero() * time.delta_seconds() * settings.speed * speed_multiplier;
    } else {
        for key in keys.get_pressed() {
            match key {
                KeyCode::Comma => velocity += forward,
                KeyCode::O => velocity -= forward,
                KeyCode::E => velocity += cam_t.right(),
                KeyCode::A => velocity += cam_t.left(),
                KeyCode::Space => velocity += Vec3::Y,
                KeyCode::ShiftLeft => velocity -= Vec3::Y,
                KeyCode::ControlLeft => speed_multiplier = 10.0,
                _ => (),
            }
        }
        if !maze_timer.player_started {
            maze_timer.player_started = true;
            maze_timer.start_time = Some(Instant::now())
        }
        let (player_t, mut controller) = player_query.single_mut();
        controller.translation = Some(
            velocity.normalize_or_zero() * time.delta_seconds() * settings.speed * speed_multiplier,
        );
        cam_t.translation = player_t.translation + Vec3::new(0.0, 0.25, 0.0);
    }
}

fn camera_look(
    mut query: Query<(&mut Transform, &CameraSettings)>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let window = window.get_single().expect("Failed to find a window");
    if window.cursor.grab_mode != CursorGrabMode::Locked {
        return;
    }

    let (mut transform, settings) = query.single_mut();
    for event in mouse_motion.iter() {
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        let window_scale = window.height().min(window.width());
        pitch -= (settings.sensitivity * event.delta.y * window_scale).to_radians();
        yaw -= (settings.sensitivity * event.delta.x * window_scale).to_radians();
        pitch = pitch.clamp(-1.54, 1.54);
        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    }
}

pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, (camera_move, camera_look));
    }
}
