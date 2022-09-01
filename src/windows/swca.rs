use std::ffi::c_void;

use winapi::shared::{basetsd::DWORD_PTR, minwindef::{ULONG, DWORD}};

pub enum ACCENT {
  AccentEnableGradient = 1, // Use a solid color specified by nColor. This mode ignores the alpha value and is fully opaque.
  AccentEnableTransparentGradient = 2, // Use a tinted transparent overlay. nColor is the tint color.
  AccentEnableBlurBehind = 3,          // Use a tinted blurry overlay. nColor is the tint color.
  AccentEnableFluent = 4, // Use an aspect similar to Fluent design. nColor is tint color. This mode bugs if the alpha value is 0.
  AccentNormal = 150,
}

impl TryFrom<i32> for ACCENT {
  type Error = ();

  fn try_from(v: i32) -> Result<Self, Self::Error> {
    match v {
      x if x == ACCENT::AccentEnableGradient as i32 => Ok(ACCENT::AccentEnableGradient),
      x if x == ACCENT::AccentEnableTransparentGradient as i32 => {
        Ok(ACCENT::AccentEnableTransparentGradient)
      }
      x if x == ACCENT::AccentEnableBlurBehind as i32 => Ok(ACCENT::AccentEnableBlurBehind),
      x if x == ACCENT::AccentEnableFluent as i32 => Ok(ACCENT::AccentEnableFluent),
      x if x == ACCENT::AccentNormal as i32 => Ok(ACCENT::AccentNormal),
      _ => Err(()),
    }
  }
}

impl Into<i32> for ACCENT {
    fn into(self) -> i32 {
      self as i32
    }
}

impl PartialEq for ACCENT {
  fn eq(&self, other: &Self) -> bool {
    core::mem::discriminant(self) == core::mem::discriminant(other)
  }
}

impl Clone for ACCENT {
  fn clone(&self) -> Self {
    match self {
      Self::AccentEnableGradient => Self::AccentEnableGradient,
      Self::AccentEnableTransparentGradient => Self::AccentEnableTransparentGradient,
      Self::AccentEnableBlurBehind => Self::AccentEnableBlurBehind,
      Self::AccentEnableFluent => Self::AccentEnableFluent,
      Self::AccentNormal => Self::AccentNormal,
    }
  }
}

pub enum WindowCompositionAttribute {
  WcaAccentPolicy = 19,
}

pub struct AccentPolicy {
  pub n_accent_state: DWORD,
  pub n_flags: DWORD,
  pub n_color: DWORD,
  pub n_animation_id: DWORD,
}

impl Clone for AccentPolicy {
  fn clone(&self) -> Self {
    Self {
      n_accent_state: self.n_accent_state.clone(),
      n_flags: self.n_flags.clone(),
      n_color: self.n_color.clone(),
      n_animation_id: self.n_animation_id.clone(),
    }
  }
}

pub struct WinCompositionData {
  pub n_attribute: DWORD_PTR,
  pub p_data: *mut c_void,
  pub ul_data_size: ULONG,
}

pub struct TaskbarAppearance {
  accent: ACCENT,
  color: u32,
}

pub const REGULAR_APPEARANCE: TaskbarAppearance = TaskbarAppearance {
  accent: ACCENT::AccentEnableTransparentGradient,
  color: 0x0,
};

pub const MAXIMISED_APPEARANCE: TaskbarAppearance = TaskbarAppearance {
  accent: ACCENT::AccentEnableBlurBehind,
  color: 0xaa000000,
};

pub const START_APPEARANCE: TaskbarAppearance = TaskbarAppearance {
  accent: ACCENT::AccentNormal,
  color: 0x0,
};

pub const CORTANA_APPEARANCE: TaskbarAppearance = TaskbarAppearance {
  accent: ACCENT::AccentNormal,
  color: 0x0,
};

pub const TIMELINE_APPEARANCE: TaskbarAppearance = TaskbarAppearance {
  accent: ACCENT::AccentNormal,
  color: 0x0,
};
