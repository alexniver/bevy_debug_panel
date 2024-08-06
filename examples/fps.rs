use std::time::Duration;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_debug_panel::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DebugPanelPlugin::new(Duration::from_millis(200)))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (trigger_fps, clear_panel))
        .run();
}

fn setup(mut commands: Commands, mut debug_res: ResMut<DebugResource>) {
    commands.spawn(Camera2dBundle { ..default() });
    commands.init_resource::<ShowFps>();
    debug_res.insert(
        "hint: ",
        "press 'f' to trigger fps, press 'c' to remove this message",
    );
}

#[derive(Debug, Resource, Default)]
struct ShowFps(bool);

fn trigger_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut debug_res: ResMut<DebugResource>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut show_fps: ResMut<ShowFps>,
) {
    if keyboard.just_pressed(KeyCode::KeyF) {
        show_fps.0 = !show_fps.0;
    }

    let k = "fps";
    if show_fps.0 {
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            debug_res.insert(k, format!("{:.2}", value));
        }
    } else if debug_res.contains_key(k) {
        debug_res.remove(k);
    }
}

fn clear_panel(mut debug_res: ResMut<DebugResource>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyC) {
        debug_res.clear();
    }
}
