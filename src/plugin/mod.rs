use std::collections::BTreeMap;
use std::time::Duration;

use bevy::prelude::*;

/// Debug Panel Plugin
/// show debug info by 'key: value'
pub struct DebugPanelPlugin;

impl Plugin for DebugPanelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugResource>();
        app.add_systems(Update, show_debug_info);
    }
}

/// Deubg Resource
#[derive(Debug, Resource)]
pub struct DebugResource {
    root_panel: Option<Entity>,
    map: BTreeMap<String, String>,
    timer: Timer,
    bg_color: Color,
    font_color: Color,
    font_size: f32,
    is_show: bool,
}

impl Default for DebugResource {
    fn default() -> Self {
        Self {
            root_panel: None,
            map: BTreeMap::new(),
            timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
            bg_color: Color::srgba(0.2, 0.7, 0.2, 0.2),
            font_color: Color::srgb(0.8, 0.8, 0.8),
            font_size: 20.,
            is_show: true,
        }
    }
}

impl DebugResource {
    /// insert debug info, key <-> value
    pub fn insert<T: Into<String>, U: Into<String>>(&mut self, k: T, v: U) {
        self.map.insert(k.into(), v.into());
    }

    /// remove debug info by key
    pub fn remove<T: Into<String>>(&mut self, k: T) -> Option<String> {
        self.map.remove(&k.into())
    }

    /// check is contain key
    pub fn contains_key<T: Into<String>>(&self, k: T) -> bool {
        self.map.contains_key(&k.into())
    }

    /// clear all debug info
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// set timer interval
    pub fn set_interval(&mut self, interval: Duration) {
        self.timer.set_duration(interval);
    }

    /// set background color
    pub fn set_bg_color(&mut self, bg_color: Color) {
        self.bg_color = bg_color;
    }

    /// set font color
    pub fn set_font_color(&mut self, font_color: Color) {
        self.font_color = font_color;
    }

    /// set font size
    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
    }

    /// set is show debug panel
    pub fn set_is_show(&mut self, is_show: bool) {
        self.is_show = is_show;
    }

    /// is show debug panel
    pub fn is_show(&self) -> bool {
        self.is_show
    }
}

fn show_debug_info(mut commands: Commands, mut debug_res: ResMut<DebugResource>, time: Res<Time>) {
    debug_res.timer.tick(time.delta());

    if debug_res.timer.finished() {
        // remove old root_panel
        if let Some(root_panel) = debug_res.root_panel {
            if let Some(root_panel) = commands.get_entity(root_panel) {
                root_panel.despawn_recursive();
            }
        }

        if debug_res.is_show {
            // generate new root_panel
            let mut root_panel = commands.spawn((
                Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(debug_res.bg_color),
            ));
            debug_res.root_panel = Some(root_panel.id());

            // generate debug info
            for (k, v) in debug_res.map.iter() {
                root_panel.with_children(|p| {
                    p.spawn((
                        Text::new(format!("{}: {}", k, v)),
                        TextFont {
                            font_size: debug_res.font_size,
                            ..default()
                        },
                        TextColor(debug_res.font_color),
                    ));
                });
            }
        }
    }
}
