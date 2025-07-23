use bevy::math::Rect;

use crate::{
  win_info::WinInfo,
  world_unit::{WorldUnit, WorldVec2},
};

#[derive(Clone)]
pub struct WorldRect {
  /// The minimum corner point of the rect.
  min: WorldVec2,
  /// The maximum corner point of the rect.
  max: WorldVec2,
}

impl WorldRect {
  pub fn from_corners(p0: WorldVec2, p1: WorldVec2) -> Self {
    Self { min: p0.min(p1), max: p0.max(p1) }
  }

  pub fn new(min: WorldVec2, max: WorldVec2) -> Self {
    Self { min, max }
  }

  pub fn from_center_size(origin: WorldVec2, size: WorldVec2) -> Self {
    let half_size = size / 2.;
    Self::from_center_half_size(origin, half_size)
  }

  pub fn from_center_half_size(origin: WorldVec2, half_size: WorldVec2) -> Self {
    Self {
      min: origin - half_size,
      max: origin + half_size,
    }
  }

  pub fn is_empty(&self) -> bool {
    self.min.x >= self.max.x || self.min.y >= self.max.y
  }

  pub fn width(&self) -> WorldUnit {
    self.max.x - self.min.x
  }

  pub fn height(&self) -> WorldUnit {
    self.max.y - self.min.y
  }

  pub fn size(&self) -> WorldVec2 {
    self.max - self.min
  }

  pub fn half_size(&self) -> WorldVec2 {
    self.size() * 0.5
  }

  pub fn center(&self) -> WorldVec2 {
    (self.min + self.max) * 0.5
  }

  pub fn contains(&self, point: WorldVec2) -> bool {
    self.min.x <= point.x && self.min.y <= point.y && self.max.x >= point.x && self.max.y >= point.y
  }

  pub fn to_rect(&self, win_info: &WinInfo) -> Rect {
    Rect::from_corners(
      self.min.to_absolute(win_info),
      self.max.to_absolute(win_info),
    )
  }
}
