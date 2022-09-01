#![deny(clippy::all)]
mod windows;

#[cfg(windows)]
#[allow(unused)]
mod exports_windows {
  use napi_derive::napi;

  use super::windows::{
    hide_desktop_icon as hide_desktop_icon_win, hide_peek_window as hide_peek_window_win,
    hide_shell_window as hide_shell_window_win, query_user_state as query_user_state_win,
    restore_window_worker as restore_window_worker_win, set_window_worker as set_window_worker_win,
    show_desktop_icon as show_desktop_icon_win, show_peek_window as show_peek_window_win,
    show_shell_window as show_shell_window_win,
  };

  #[napi]
  pub fn set_window_worker(h_wnd: u32) {
    set_window_worker_win(h_wnd as *const usize);
  }

  #[napi]
  pub fn restore_window_worker() {
    restore_window_worker_win()
  }

  #[napi]
  pub fn show_desktop_icon() {
    show_desktop_icon_win();
  }

  #[napi]
  pub fn hide_desktop_icon() {
    hide_desktop_icon_win();
  }

  #[napi]
  pub fn show_shell_window() {
    show_shell_window_win()
  }

  #[napi]
  pub fn hide_shell_window() {
    hide_shell_window_win()
  }

  #[napi]
  pub fn show_peek_window() {
    show_peek_window_win()
  }

  #[napi]
  pub fn hide_peek_window() {
    hide_peek_window_win()
  }

  #[napi]
  pub fn query_user_state() -> u32 {
    query_user_state_win()
  }
}

mod exports_common {
  use napi::bindgen_prelude::*;
  use napi_derive::napi;

  #[napi]
  pub enum UserState {
    QunsNotPresent = 1,
    QunsBusy = 2,
    QunsRunningD3dFullScreen = 3,
    QunsPresentationMode = 4,
    QunsAcceptsNotifications = 5,
    QunsQuietTime = 6,
    QunsApp = 7,
  }

  #[napi]
  pub enum ACCENT {
    AccentEnableGradient = 1, // Use a solid color specified by nColor. This mode ignores the alpha value and is fully opaque.
    AccentEnableTransparentgradient = 2, // Use a tinted transparent overlay. nColor is the tint color.
    AccentEnableBlurbehind = 3,          // Use a tinted blurry overlay. nColor is the tint color.
    AccentEnableFluent = 4, // Use an aspect similar to Fluent design. nColor is tint color. This mode bugs if the alpha value is 0.
    AccentNormal = 150,
  }
}

mod exports_linux {}

mod exports_macos {}

#[cfg(not(windows))]
#[napi]
pub fn set_window_worker(h_wnd: u32) {
  todo!()
}

#[cfg(not(windows))]
#[napi]
pub fn show_desktop_icon() {
  todo!()
}

#[cfg(not(windows))]
#[napi]
pub fn hide_desktop_icon() {
  todo!()
}
