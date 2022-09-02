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

#[napi(object)]
#[derive(Debug)]
pub struct RECT {
  pub top: i32,
  pub left: i32,
  pub right: i32,
  pub bottom: i32,
}

#[napi]
#[derive(Debug)]
pub enum Alignment {
  Left,
  Top,
  Right,
  Bottom,
}

#[napi(object)]
#[derive(Debug)]
pub struct TaskbarState {
  pub rc: RECT,
  pub alignment: Alignment,
}

impl TaskbarState {
  pub fn new() -> TaskbarState {
    TaskbarState {
      rc: RECT {
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
      },
      alignment: Alignment::Left,
    }
  }
}

#[napi]
pub enum ACCENT {
  AccentEnableGradient = 1, // Use a solid color specified by nColor. This mode ignores the alpha value and is fully opaque.
  AccentEnableTransparentgradient = 2, // Use a tinted transparent overlay. nColor is the tint color.
  AccentEnableBlurbehind = 3,          // Use a tinted blurry overlay. nColor is the tint color.
  AccentEnableFluent = 4, // Use an aspect similar to Fluent design. nColor is tint color. This mode bugs if the alpha value is 0.
  AccentNormal = 150,
}

impl Into<u32> for ACCENT {
  fn into(self) -> u32 {
    self as u32
  }
}

#[napi(object)]
pub struct Color {
  pub r: u32,
  pub g: u32,
  pub b: u32,
  pub a: u32,
}

impl Color {
  pub fn new(r: Option<u32>, g: Option<u32>, b: Option<u32>, a: Option<u32>) -> Color {
    let _r = if let Some(r) = r { r } else { 0 };
    let _g = if let Some(g) = g { g } else { 0 };
    let _b = if let Some(b) = b { b } else { 0 };
    let _a = if let Some(a) = a { a } else { 0 };

    Color {
      r: _r,
      g: _g,
      b: _b,
      a: _a,
    }
  }

  pub fn to_argb(&self) -> u32 {
    self.a << 24 | self.r << 16 | self.g << 8 | self.b << 0
  }
}
