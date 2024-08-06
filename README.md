# bevy_debug_panel

add debug info to screen

[examples](./examples/)

## Quickstart

simple show framerate example

```rs
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_debug_panel::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DebugPanelPlugin::new(Duration::from_millis(200)))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (show_fps))
        .run();
}

fn setup(mut commands: Commands, mut debug_res: ResMut<DebugResource>) {
    commands.spawn(Camera2dBundle { ..default() });
}

fn show_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut debug_res: ResMut<DebugResource>,
) {
    if let Some(value) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        debug_res.insert("fps", format!("{:.2}", value));
    }
}

```


## compatible bevy versions

| bevy | bevy_debug_panel |
| ---- | ----------------------- |
| 0.14 | 0.14                     |
