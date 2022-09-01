#![deny(clippy::all)]
mod windows;

use napi::bindgen_prelude::*;
use napi_derive::napi;

#[cfg(windows)]
use windows::{
  hide_desktop_icon as hide_desktop_icon_win, set_window_worker as set_window_worker_win,
  show_desktop_icon as show_desktop_icon_win, restore_window_worker as restore_window_worker_win
};

#[cfg(windows)]
#[napi]
pub fn set_window_worker(h_wnd: u32) {
  set_window_worker_win(h_wnd as *const usize);
}

#[cfg(windows)]
#[napi]
pub fn restore_window_worker() {
	restore_window_worker_win()
}

#[cfg(windows)]
#[napi]
pub fn show_desktop_icon() {
  show_desktop_icon_win();
}

#[cfg(windows)]
#[napi]
pub fn hide_desktop_icon() {
  hide_desktop_icon_win();
}

#[napi]
pub enum UserState {
  QunsNotPresent = 1,
  QunsBusy = 2,
  QunsRunningD3dFullScreen = 3,
  QunsPresentationMode = 4,
  QunsAcceptsNotifications = 5,
  QunsQuietTime = 6,
  QunsApp = 7
}

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
