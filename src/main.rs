use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
mod camera;
use camera::FlyCamera;
use camera::FlyCameraPlugin;
mod maze2;
use maze2::MazePlugin;
//mod scene;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use scene::ScenePlugin;

fn exit_on_escape(mut exit: EventWriter<AppExit>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn spawn_on_t(
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

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            FlyCameraPlugin,
            MazePlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            //RapierDebugRenderPlugin::default(),
            //           ScenePlugin,
        ))
        .add_systems(Update, (exit_on_escape, spawn_on_t))
        .run();
}
