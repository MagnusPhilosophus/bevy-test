use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
mod camera;
use camera::CameraSettings;
use camera::FlyCameraPlugin;
mod maze;
use maze::MazePlugin;
mod scene;
use scene::ScenePlugin;
mod ui;
use ui::UIPlugin;
mod utils;
use utils::UtilsPlugin;

fn spawn_on_e(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<Input<KeyCode>>,
    position: Query<&Transform, With<CameraSettings>>,
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
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

fn spawn_on_q(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<Input<KeyCode>>,
    position: Query<&Transform, With<CameraSettings>>,
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
            ActiveEvents::COLLISION_EVENTS,
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
        KinematicCharacterController {
            apply_impulse_to_dynamic_bodies: true,
            //filter_flags: bevy_rapier3d::pipeline::QueryFilterFlags::EXCLUDE_SENSORS,
            ..default()
        },
        Name::new("Player"),
    ));
}

fn collisions_printer(mut outputs: Query<&mut KinematicCharacterControllerOutput>) {
    for output in outputs.iter_mut() {
        for collision in &output.collisions {
            println!("Player collided with {:?}", collision.entity);
        }
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

#[cfg(debug_assertions)]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlyCameraPlugin,
            MazePlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            UIPlugin,
            ScenePlugin,
            UtilsPlugin,
        ))
        .add_systems(Startup, setup_player)
        .add_systems(
            Update,
            (spawn_on_q, spawn_on_e, collisions_printer, display_events),
        )
        .run();
}

#[cfg(not(debug_assertions))]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlyCameraPlugin,
            MazePlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            //RapierDebugRenderPlugin::default(),
            UIPlugin,
            ScenePlugin,
            UtilsPlugin,
        ))
        .add_systems(Startup, setup_player)
        .add_systems(Update, (spawn_on_q, spawn_on_e))
        .run();
}
