#![deny(clippy::all)]
mod common;
#[cfg(windows)]
mod windows;

#[cfg(windows)]
#[allow(unused)]
mod exports_windows {
  use crate::common::{
    Color, CursorResourceCollection, InternalCursorResourceCollection, TaskbarState, ACCENT, RECT,
  };

  use super::windows::{
    get_sys_list_view_icon_rect as get_sys_list_view_icon_rect_win,
    get_sys_taskbar_state as get_sys_taskbar_state_win, hide_desktop_icon as hide_desktop_icon_win,
    hide_peek_window as hide_peek_window_win, hide_shell_window as hide_shell_window_win,
    is_in_desktop_window as is_in_desktop_window_win, query_user_state as query_user_state_win,
    restore_system_cursor_style as restore_system_cursor_style_win,
    restore_taskbar_style as restore_taskbar_style_win,
    restore_window_worker as restore_window_worker_win,
    set_system_cursor_style as set_system_cursor_style_win,
    set_taskbar_style as set_taskbar_style_win, set_window_worker as set_window_worker_win,
    show_desktop_icon as show_desktop_icon_win, show_peek_window as show_peek_window_win,
    show_shell_window as show_shell_window_win,
  };
  use napi_derive::napi;

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

  #[napi]
  pub fn set_taskbar_style(accept: ACCENT, color: Color) -> bool {
    set_taskbar_style_win(accept, color.to_argb())
  }

  #[napi]
  pub fn restore_taskbar_style() -> bool {
    restore_taskbar_style_win()
  }

  #[napi]
  pub fn get_sys_list_view_icon_rect() -> Vec<RECT> {
    get_sys_list_view_icon_rect_win()
  }

  #[napi]
  pub fn get_sys_taskbar_state() -> TaskbarState {
    get_sys_taskbar_state_win()
  }

  #[napi]
  pub fn set_system_cursor_style(resource: CursorResourceCollection) {
    let internal_resource = InternalCursorResourceCollection {
      app_starting: Some(resource.app_starting),
      arrow: Some(resource.arrow),
      hand: Some(resource.hand),
      cross: Some(resource.cross),
      wait: Some(resource.wait),
      i_beam: Some(resource.i_beam),
      no: Some(resource.no),
      size: Some(resource.size),
      size_all: Some(resource.size_all),
      size_nesw: Some(resource.size_nesw),
      size_ns: Some(resource.size_ns),
      size_nwse: Some(resource.size_nwse),
      size_we: Some(resource.size_we),
      up_arrow: Some(resource.up_arrow),
      help: Some(resource.help),
    };

    set_system_cursor_style_win(internal_resource)
  }

  #[napi]
  pub fn restore_system_cursor_style() {
    restore_system_cursor_style_win()
  }

  #[napi]
  pub fn is_in_desktop_window() -> bool {
    is_in_desktop_window_win()
  }
}

#[cfg(linux)]
mod exports_linux {}

#[cfg(macos)]
mod exports_macos {}

#[cfg(not(windows))]
#[allow(unused)]
mod exports_not_windows {
  use crate::common::{Color, TaskbarState, ACCENT};
  use napi_derive::napi;

  #[napi]
  pub fn set_window_worker(h_wnd: u32) {
    todo!()
  }

  #[napi]
  pub fn restore_window_worker() {
    todo!()
  }

  #[napi]
  pub fn show_desktop_icon() {
    todo!()
  }

  #[cfg(not(windows))]
  #[napi]
  pub fn hide_desktop_icon() {
    todo!()
  }

  #[napi]
  pub fn show_shell_window() {
    todo!()
  }

  #[napi]
  pub fn hide_shell_window() {
    todo!()
  }

  #[napi]
  pub fn show_peek_window() {
    todo!()
  }

  #[napi]
  pub fn hide_peek_window() {
    todo!()
  }

  #[napi]
  pub fn query_user_state() -> u32 {
    5
  }

  #[napi]
  pub fn set_taskbar_style(accept: ACCENT, color: Color) -> bool {
    true
  }

  #[napi]
  pub fn restore_taskbar_style() -> bool {
    true
  }

  #[napi]
  pub fn get_sys_taskbar_state() -> TaskbarState {
    TaskbarState::new()
  }

  #[napi]
  pub fn set_system_cursor_style() {
    todo!()
  }

  #[napi]
  pub fn restore_system_cursor_style() {
    todo!()
  }

  #[napi]
  pub fn is_in_desktop_window() -> bool {
    false
  }
}
