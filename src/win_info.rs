use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct WinInfo {
  pub width: f32,
  pub height: f32,
}

impl WinInfo {
  pub fn new(width: f32, height: f32) -> Self {
    WinInfo { width, height }
  }
}
