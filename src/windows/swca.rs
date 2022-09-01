use winapi::ctypes::c_void;

enum ACCENT {
  ACCENT_ENABLE_GRADIENT = 1, // Use a solid color specified by nColor. This mode ignores the alpha value and is fully opaque.
  ACCENT_ENABLE_TRANSPARENTGRADIENT = 2, // Use a tinted transparent overlay. nColor is the tint color.
  ACCENT_ENABLE_BLURBEHIND = 3,          // Use a tinted blurry overlay. nColor is the tint color.
  ACCENT_ENABLE_FLUENT = 4, // Use an aspect similar to Fluent design. nColor is tint color. This mode bugs if the alpha value is 0.
  ACCENT_NORMAL = 150,
}

enum WindowCompositionAttribute {
  WCA_ACCENT_POLICY = 19,
}

struct ACCENTPOLICY {
  nAccentState: ACCENT,
  nFlags: i32,
  nColor: u32,
  nAnimationId: i32,
}

pub struct WINCOMPATTRDATA {
  nAttribute: WindowCompositionAttribute,
  pData: *mut c_void,
  ulDataSize: u32,
}

pub struct TASKBAR_APPEARANCE {
  ACCENT: ACCENT,
  COLOR: u32,
}

pub const REGULAR_APPEARANCE: TASKBAR_APPEARANCE = TASKBAR_APPEARANCE {
  ACCENT: ACCENT::ACCENT_ENABLE_TRANSPARENTGRADIENT,
  COLOR: 0x0,
};

pub const MAXIMISED_APPEARANCE: TASKBAR_APPEARANCE = TASKBAR_APPEARANCE {
  ACCENT: ACCENT::ACCENT_ENABLE_BLURBEHIND,
  COLOR: 0xaa000000,
};

pub const START_APPEARANCE: TASKBAR_APPEARANCE = TASKBAR_APPEARANCE {
  ACCENT: ACCENT::ACCENT_NORMAL,
  COLOR: 0x0,
};

pub const CORTANA_APPEARANCE: TASKBAR_APPEARANCE = TASKBAR_APPEARANCE {
  ACCENT: ACCENT::ACCENT_NORMAL,
  COLOR: 0x0,
};

pub const TIMELINE_APPEARANCE: TASKBAR_APPEARANCE = TASKBAR_APPEARANCE {
  ACCENT: ACCENT::ACCENT_NORMAL,
  COLOR: 0x0,
};
