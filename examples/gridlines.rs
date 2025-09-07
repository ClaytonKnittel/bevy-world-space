use bevy::{
  DefaultPlugins, app::App, color::Color, prelude::PluginGroup, render::camera::ClearColor,
};
use bevy_world_space::{WorldSpacePlugins, world_init::WorldInitPlugin};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    // .insert_resource(ClearColor(Color::srgb(
    //   178. / 255.,
    //   216. / 255.,
    //   216. / 255.,
    // )))
    .add_plugins(
      WorldSpacePlugins.set(WorldInitPlugin { screen_width: 1280., screen_height: 720. }),
    )
    .run();
}
