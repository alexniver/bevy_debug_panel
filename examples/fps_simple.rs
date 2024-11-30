use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_debug_panel::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DebugPanelPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
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
