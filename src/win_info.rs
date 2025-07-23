use bevy::ecs::system::Resource;

#[derive(Clone, Copy)]
pub struct AspectRatio(pub(crate) f32);

#[derive(Resource)]
pub struct WinInfo {
  aspect_ratio: AspectRatio,
  width: f32,
  height: f32,
}

impl WinInfo {
  pub(crate) fn new(width: f32, height: f32) -> Self {
    Self {
      aspect_ratio: AspectRatio(height / width),
      width,
      height,
    }
  }

  pub fn aspect_ratio(&self) -> AspectRatio {
    self.aspect_ratio
  }

  pub fn width(&self) -> f32 {
    self.width
  }

  pub(crate) fn set_width(&mut self, width: f32) {
    self.width = width;
  }

  pub fn height(&self) -> f32 {
    self.height
  }

  pub(crate) fn set_height(&mut self, height: f32) {
    self.height = height;
  }
}
