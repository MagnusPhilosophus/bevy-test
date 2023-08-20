use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use std::time::Instant;

#[derive(Component)]
struct TextChanges;

#[derive(Component)]
struct MazeTimerText;

#[derive(Resource)]
pub struct MazeTimer {
    pub player_started: bool,
    pub start_time: Option<Instant>,
}

fn info_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraCode-Bold.ttf");
    commands.spawn((
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::WHITE,
            }),
            TextSection::new(
                " fps\n",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::WHITE,
            }),
            TextSection::new(
                " ms/frame",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::BLUE,
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        TextChanges,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "There should be a ",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "timer",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        MazeTimerText,
    ));

    commands.insert_resource(MazeTimer {
        player_started: false,
        start_time: None,
    });
}

fn maze_timer_update(maze_timer: Res<MazeTimer>, mut text: Query<&mut Text, With<MazeTimerText>>) {
    if maze_timer.player_started {
        let mut text = text.single_mut();
        let elapsed = Instant::now().duration_since(maze_timer.start_time.unwrap());
        text.sections[1].value = format!("{:.2}", elapsed.as_secs_f32());
    }
}

fn change_text(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<TextChanges>>,
) {
    for mut text in &mut query {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                frame_time = frame_time_smoothed;
            }
        }

        text.sections[0].value = format!("{fps:.1}");

        text.sections[2].value = format!("{frame_time:.3}");
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, info_text)
            .add_systems(Update, (change_text, maze_timer_update));
    }
}
