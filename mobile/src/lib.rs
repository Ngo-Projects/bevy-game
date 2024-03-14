use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy-game::GamePlugin; // ToDo: Replace bevy-game with your new crate name.

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
        ))
        .run()
}
