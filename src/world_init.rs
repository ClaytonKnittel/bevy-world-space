#[cfg(not(target_arch = "wasm32"))]
use bevy::{
  app::AppExit,
  ecs::{event::EventWriter, system::Res},
  input::{ButtonInput, keyboard::KeyCode},
};
use bevy::{
  app::{App, Plugin, Startup, Update},
  core_pipeline::core_2d::Camera2d,
  ecs::{
    event::EventReader,
    system::{Commands, ResMut},
  },
  window::WindowResized,
};

use crate::{win_info::WinInfo, world_unit::AspectRatio};

pub struct WorldInitPlugin {
  pub screen_width: f32,
  pub screen_height: f32,
}

impl Default for WorldInitPlugin {
  fn default() -> Self {
    Self { screen_width: 1280., screen_height: 720. }
  }
}

impl WorldInitPlugin {
  pub fn world_init(mut commands: Commands) {
    commands.spawn(Camera2d);
  }

  #[cfg(not(target_arch = "wasm32"))]
  fn app_exit_listener(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit: EventWriter<AppExit>,
  ) {
    if keyboard_input.pressed(KeyCode::Escape) {
      app_exit.send(AppExit::Success);
    }
  }

  #[cfg(target_arch = "wasm32")]
  fn app_exit_listener() {}

  fn resize_listener(mut resize_events: EventReader<WindowResized>, mut win_info: ResMut<WinInfo>) {
    for e in resize_events.read() {
      win_info.width = e.width;
      win_info.height = e.height;
    }
  }
}

impl Plugin for WorldInitPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(WinInfo::new(self.screen_width, self.screen_height))
      .insert_resource(AspectRatio::new(self.screen_height / self.screen_width))
      .add_systems(Startup, Self::world_init)
      .add_systems(Update, (Self::app_exit_listener, Self::resize_listener));
  }
}
