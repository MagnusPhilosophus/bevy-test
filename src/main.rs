use bevy::app::AppExit;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
mod camera;
use camera::FlyCamera;
use camera::FlyCameraPlugin;
mod maze2;
use maze2::MazePlugin;
mod scene;
use scene::ScenePlugin;
mod ui;
use std::time::Instant;
use ui::MazeTimer;
use ui::UIPlugin;

#[derive(Component)]
pub struct PlayerCamera;

fn exit_on_escape(mut exit: EventWriter<AppExit>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn spawn_on_e(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<Input<KeyCode>>,
    position: Query<&Transform, With<FlyCamera>>,
) {
    if keys.just_pressed(KeyCode::Period) {
        let position = position.single();
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.05 })),
                material: materials.add(Color::rgb(0.2, 0.5, 0.5).into()),
                transform: position.clone(),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(0.05, 0.05, 0.05),
            Restitution::coefficient(0.7),
            Velocity {
                linvel: position.forward() * 10.0,
                ..default()
            },
        ));
    }
}

fn spawn_on_q(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<Input<KeyCode>>,
    position: Query<&Transform, With<FlyCamera>>,
) {
    if keys.just_pressed(KeyCode::Semicolon) {
        let position = position.single();
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(
                    shape::Icosphere {
                        radius: 0.25,
                        ..default()
                    }
                    .try_into()
                    .unwrap(),
                ),
                material: materials.add(Color::rgb(0.2, 0.5, 0.5).into()),
                transform: position.clone(),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(0.25),
            Restitution::coefficient(0.7),
            Velocity {
                linvel: position.forward() * 10.0,
                ..default()
            },
        ));
    }
}

fn setup_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut transform = Transform::from_xyz(0.5, 0.5, 0.5);
    transform.rotate_local_y(f32::to_radians(180.0));
    // Player
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                shape::Capsule {
                    radius: 0.25,
                    depth: 0.5,
                    ..default()
                }
                .try_into()
                .unwrap(),
            ),
            material: materials.add(Color::rgb(0.2, 0.5, 0.5).into()),
            transform,
            ..default()
        },
        RigidBody::KinematicPositionBased,
        Collider::capsule(Vec3::new(0.0, -0.25, 0.0), Vec3::new(0.0, 0.25, 0.0), 0.25),
        KinematicCharacterController::default(),
    ));
    commands.spawn((Camera3dBundle::default(), PlayerCamera));
}

fn update_player_camera(
    mut camera: Query<&mut Transform, With<PlayerCamera>>,
    player: Query<&Transform, (With<KinematicCharacterController>, Without<PlayerCamera>)>,
) {
    let mut camera = camera.single_mut();
    let player = player.single();
    camera.translation = player.translation + Vec3::new(0.0, 0.25, 0.0);
}

fn update_player(
    mut query: Query<(&Transform, &mut KinematicCharacterController)>,
    mut maze_timer: ResMut<MazeTimer>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if keys.any_pressed([KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right])
        && !maze_timer.player_started
    {
        maze_timer.player_started = true;
        maze_timer.start_time = Some(Instant::now())
    }
    let (transform, mut controller) = query.single_mut();
    let mut velocity = Vec3::ZERO;
    for key in keys.get_pressed() {
        match key {
            KeyCode::Up => velocity += transform.forward(),
            KeyCode::Down => velocity += transform.back(),
            KeyCode::Left => velocity += transform.left(),
            KeyCode::Right => velocity += transform.right(),
            _ => (),
        }
    }
    controller.translation = Some(velocity.normalize_or_zero() * time.delta_seconds());
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            FlyCameraPlugin,
            MazePlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            UIPlugin,
            ScenePlugin,
        ))
        .add_systems(Startup, setup_player)
        .add_systems(
            Update,
            (
                exit_on_escape,
                spawn_on_q,
                spawn_on_e,
                update_player,
                update_player_camera,
            ),
        )
        .run();
}
