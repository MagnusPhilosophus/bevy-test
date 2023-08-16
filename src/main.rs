use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
mod camera;
use camera::FlyCamera;
use camera::FlyCameraPlugin;
mod maze2;
use maze2::MazePlugin;
mod scene;
//use scene::ScenePlugin;

fn exit_on_escape(mut exit: EventWriter<AppExit>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn setup_physics(mut commands: Commands) {
    commands.spawn((
        Collider::cuboid(50.0, 0.1, 50.0),
        TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)),
    ));
}

fn spawn_on_t(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    position: Query<&Transform, With<FlyCamera>>,
) {
    if keys.just_pressed(KeyCode::T) {
        let position = position.single();
        commands.spawn((
            RigidBody::Dynamic,
            Collider::ball(0.5),
            Restitution::coefficient(0.7),
            TransformBundle::from_transform(position.clone()),
            Velocity {
                linvel: position.forward() * 50.0,
                ..default()
            },
        ));
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlyCameraPlugin,
            MazePlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, setup_physics)
        .add_systems(Update, (exit_on_escape, spawn_on_t))
        .run();
}
