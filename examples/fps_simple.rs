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
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
        .add_systems(Startup, setup)
        .add_systems(Update, show_fps)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn show_fps(diagnostics: Res<DiagnosticsStore>, mut debug_res: ResMut<DebugResource>) {
    if let Some(value) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        debug_res.insert("Fps", format!("{:.2}", value));
    }
}
