use bevy::app::AppExit;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn exit_on_escape(mut exit: EventWriter<AppExit>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
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

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FrameTimeDiagnosticsPlugin, WorldInspectorPlugin::new()))
            .add_systems(Startup, initial_grab_cursor)
            .add_systems(Update, (exit_on_escape, grab_cursor));
    }
}
