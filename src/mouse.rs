use bevy::{
  app::{App, Plugin, PreUpdate},
  ecs::{
    event::{Event, EventWriter},
    query::With,
    system::{Res, Single},
  },
  input::{ButtonInput, mouse::MouseButton},
  window::{PrimaryWindow, Window},
};

use crate::{
  win_info::WinInfo,
  world_unit::{AspectRatio, WorldVec2},
};

#[derive(Event)]
pub enum MouseEvent {
  LeftClick(WorldVec2),
  RightClick(WorldVec2),
}

#[derive(Default)]
pub(crate) struct MousePlugin;

impl MousePlugin {
  fn handle_input(
    win_info: Res<WinInfo>,
    aspect_ratio: Res<AspectRatio>,
    mut mouse_events: EventWriter<MouseEvent>,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
  ) {
    let Some(cursor_pos) = window.cursor_position() else {
      return;
    };

    let pos = WorldVec2::from_window_screen_pos(cursor_pos, &win_info, &aspect_ratio);
    if buttons.just_pressed(MouseButton::Left) {
      mouse_events.send(MouseEvent::LeftClick(pos));
    }
    if buttons.just_pressed(MouseButton::Right) {
      mouse_events.send(MouseEvent::RightClick(pos));
    }
  }
}

impl Plugin for MousePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(PreUpdate, MousePlugin::handle_input)
      .add_event::<MouseEvent>();
  }
}
