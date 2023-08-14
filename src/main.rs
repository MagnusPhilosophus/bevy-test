use bevy::app::AppExit;
use bevy::prelude::*;
mod camera;
use camera::FlyCameraPlugin;
mod maze;
use maze::MazePlugin;
mod scene;
//use scene::ScenePlugin;

fn exit_on_escape(mut exit: EventWriter<AppExit>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FlyCameraPlugin, MazePlugin))
        .add_systems(Update, exit_on_escape)
        .run();
}
