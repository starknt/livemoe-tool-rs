use super::macros::get_function_impl;
use crate::get_function;
use lazy_static::lazy_static;
use winapi::shared::{
  basetsd::SIZE_T,
  minwindef::{BOOL, DWORD, FARPROC},
  ntdef::PVOID,
  windef::HWND,
};

#[allow(non_snake_case)]
pub type WINDOWCOMPOSITIONATTRIB = u32;

#[allow(non_snake_case)]
#[repr(C)]
pub struct WINDOWCOMPOSITIONATTRIBDATA {
  pub Attrib: WINDOWCOMPOSITIONATTRIB,
  pub pvData: PVOID,
  pub cbData: SIZE_T,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct ACCENTPOLICY {
  pub nAccentState: DWORD,
  pub nFlags: DWORD,
  pub nColor: DWORD,
  pub nAnimationId: DWORD,
}

impl Clone for ACCENTPOLICY {
  fn clone(&self) -> Self {
    Self {
      nAccentState: self.nAccentState.clone(),
      nFlags: self.nFlags.clone(),
      nColor: self.nColor.clone(),
      nAnimationId: self.nAnimationId.clone(),
    }
  }
}

pub type SetWindowCompositionAttribute =
  unsafe extern "system" fn(HWND, *mut WINDOWCOMPOSITIONATTRIBDATA) -> BOOL;

pub fn get_set_window_composition_attribute_func() -> Option<SetWindowCompositionAttribute> {
  lazy_static! {
    static ref SET_WINDOW_COMPOSITION_ATTRIBUTE: Option<SetWindowCompositionAttribute> =
      get_function!("user32.dll", SetWindowCompositionAttribute);
  }

  *SET_WINDOW_COMPOSITION_ATTRIBUTE
}
