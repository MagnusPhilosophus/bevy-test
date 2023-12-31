#![allow(dead_code)]
use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
struct Turret {
    shooting_timer: Timer,
}

#[derive(Component)]
struct Bullet {
    lifetime_timer: Timer,
}

fn setup_light(mut commands: Commands) {
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

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //asset_server: Res<AssetServer>,
) {
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..default()
        },
        Turret {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        },
    ));
}

fn rotate_cube(mut cube_transform: Query<&mut Transform, With<Turret>>, time: Res<Time>) {
    cube_transform.single_mut().rotate_y(time.delta_seconds());
}

fn tower_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut turrets: Query<&mut Turret>,
    time: Res<Time>,
) {
    for mut turret in turrets.iter_mut() {
        turret.shooting_timer.tick(time.delta());
        if turret.shooting_timer.just_finished() {
            commands.spawn((
                PbrBundle {
                    transform: Transform::from_xyz(2.0, 0.0, 0.0),
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                    material: materials.add(Color::rgb(0.87, 0.44, 0.42).into()),
                    ..default()
                },
                Bullet {
                    lifetime_timer: Timer::from_seconds(0.5, TimerMode::Once),
                },
            ));
        }
    }
}

fn bullets_dispawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Bullet)>,
    time: Res<Time>,
) {
    for (bullet_entity, mut bullet) in bullets.iter_mut() {
        bullet.lifetime_timer.tick(time.delta());
        if bullet.lifetime_timer.just_finished() {
            commands.entity(bullet_entity).despawn();
        }
    }
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_light);
        //.add_systems(Update, (rotate_cube, tower_shooting, bullets_dispawn));
    }
}
