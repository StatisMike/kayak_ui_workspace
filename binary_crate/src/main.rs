use bevy::{prelude::{App, Commands, Camera2dBundle}, DefaultPlugins};
use library_crate::UiPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(startup)
    .add_plugin(UiPlugin)
    .run();
}

fn startup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}