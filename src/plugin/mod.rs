use std::collections::BTreeMap;
use std::time::Duration;

use bevy::prelude::*;

/// Debug Panel Plugin
pub struct DebugPanelPlugin {
    /// Debug Panel update interval
    interval: Duration,
}

impl DebugPanelPlugin {
    pub fn new(interval: Duration) -> Self {
        Self { interval }
    }
}

impl Default for DebugPanelPlugin {
    fn default() -> Self {
        Self {
            interval: Duration::from_millis(500),
        }
    }
}

impl Plugin for DebugPanelPlugin {
    fn build(&self, app: &mut App) {
        let timer = Timer::new(self.interval, TimerMode::Repeating);
        let debug_res = DebugResource { timer, ..default() };
        app.insert_resource(debug_res);
        app.add_systems(Startup, setup_root_panel);
        app.add_systems(Update, show_debug_info);
    }
}

#[derive(Debug, Resource, Default)]
pub struct DebugResource {
    root_panel: Option<Entity>,
    map: BTreeMap<String, String>,
    timer: Timer,
}

impl DebugResource {
    pub fn insert<T: Into<String>, U: Into<String>>(&mut self, k: T, v: U) {
        self.map.insert(k.into(), v.into());
    }

    pub fn remove<T: Into<String>>(&mut self, k: T) -> Option<String> {
        self.map.remove(&k.into())
    }

    pub fn contains_key<T: Into<String>>(&self, k: T) -> bool {
        self.map.contains_key(&k.into())
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

fn setup_root_panel(mut commands: Commands, mut debug_res: ResMut<DebugResource>) {
    let entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                // justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .id();
    debug_res.root_panel = Some(entity);
}

fn show_debug_info(mut commands: Commands, mut debug_res: ResMut<DebugResource>, time: Res<Time>) {
    debug_res.timer.tick(time.delta());

    if debug_res.timer.finished() {
        if let Some(root_panel) = debug_res.root_panel {
            if let Some(mut root_panel) = commands.get_entity(root_panel) {
                root_panel.despawn_descendants();
                for (k, v) in debug_res.map.iter() {
                    root_panel.with_children(|p| {
                        p.spawn(TextBundle::from_section(
                            format!("{}: {}", k, v),
                            TextStyle {
                                font_size: 30.0,
                                ..default()
                            },
                        ));
                    });
                }
            }
        }
    }
}
