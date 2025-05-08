use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_debug_panel::prelude::*;

const MAX_HISTORY_LENGTH: usize = 16;
const SMOOTHING_FACTOR: f64 = 2.0 / (MAX_HISTORY_LENGTH + 1) as f64;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DebugPanelPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin {
            max_history_length: MAX_HISTORY_LENGTH,
            smoothing_factor: SMOOTHING_FACTOR,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (trigger_fps, show_hide_panel))
        .run();
}

fn setup(mut commands: Commands, mut debug_res: ResMut<DebugResource>) {
    commands.spawn(Camera2d);
    commands.init_resource::<ShowFps>();
    debug_res.insert(
        "hint: ",
        "press 'f' to trigger fps, press 'h' to hide/show debug panel",
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

fn show_hide_panel(mut debug_res: ResMut<DebugResource>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        let is_show = debug_res.is_show();
        debug_res.set_is_show(!is_show);
    }
}
