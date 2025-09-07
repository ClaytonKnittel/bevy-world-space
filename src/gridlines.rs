use bevy::{
  color::palettes::css::{LIME, RED},
  prelude::*,
};

use crate::{win_info::WinInfo, world_unit::WorldUnit};

#[derive(Default, Resource)]
struct GridlinesResource {
  enabled: bool,
}

fn run_if_enabled(gridlines_resource: Res<GridlinesResource>) -> bool {
  true //|| gridlines_resource.enabled
}

pub struct GridlinesPlugin {
  pub toggle_key: KeyCode,
}

impl GridlinesPlugin {
  pub const DEFAULT_KEY: KeyCode = KeyCode::KeyL;

  fn toggle(mut gridlines_resource: ResMut<GridlinesResource>) {
    gridlines_resource.enabled = !gridlines_resource.enabled;
  }

  fn render(mut gizmos: Gizmos, win_info: Res<WinInfo>, time: Res<Time>) {
    let sin_t_scaled = bevy::prelude::ops::sin(time.elapsed_secs()) * 50.;
    gizmos.line_2d(Vec2::Y * -sin_t_scaled, Vec2::splat(-80.), RED);
    gizmos.ray_2d(Vec2::Y * sin_t_scaled, Vec2::splat(80.), LIME);

    let max_units = WorldUnit::UNITS_PER_SCREEN_MAX.ceil() as u32;
    for row in 0..max_units {
      let width = win_info.width();
      let height = win_info.height();

      let y = (row as f32 / WorldUnit::UNITS_PER_SCREEN_MAX - 0.5) * height;
      gizmos.line_2d(Vec2::new(-width / 2., y), Vec2::new(width / 2., y), RED);
    }

    for col in 0..max_units {
      let width = win_info.width();
      let height = win_info.height();

      let x = (col as f32 / WorldUnit::UNITS_PER_SCREEN_MAX - 0.5) * width;
      gizmos.line_2d(Vec2::new(x, -height / 2.), Vec2::new(x, height / 2.), RED);
    }
  }
}

impl Default for GridlinesPlugin {
  fn default() -> Self {
    Self { toggle_key: Self::DEFAULT_KEY }
  }
}

impl Plugin for GridlinesPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<GridlinesResource>()
      .add_systems(Update, (Self::toggle, Self::render).chain());
  }
}
