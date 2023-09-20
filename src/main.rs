use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
mod player;
use player::PlayerPlugin;
mod camera;
use camera::FlyCameraPlugin;
mod maze;
use maze::MazePlugin;
mod scene;
use scene::ScenePlugin;
mod ui;
use ui::UIPlugin;
mod utils;
use utils::UtilsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            FlyCameraPlugin,
            MazePlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            UIPlugin,
            ScenePlugin,
            UtilsPlugin,
        ))
        .run();
}
