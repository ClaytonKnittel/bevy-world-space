use bevy::app::plugin_group;

pub mod position;
pub mod win_info;
pub mod world_init;
pub mod world_unit;

plugin_group! {
  pub struct WorldSpacePlugins {
    position:::PositionPlugin,
    world_init:::WorldInitPlugin,
  }
}
