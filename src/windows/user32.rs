use super::macros::get_function_impl;
use crate::get_function;
use lazy_static::lazy_static;
use wchar::{wchar_t, wchz};
use windows::Win32::Foundation::{HWND, BOOL};
use std::ffi::{ c_void, c_ulong };

#[allow(non_snake_case)]
pub type WINDOWCOMPOSITIONATTRIB = u32;

const LIB_NAME: &[wchar_t] = wchz!("user32.dll");

#[allow(non_snake_case)]
#[repr(C)]
pub struct WINDOWCOMPOSITIONATTRIBDATA {
  pub Attrib: WINDOWCOMPOSITIONATTRIB,
  pub pvData: *mut c_void,
  pub cbData: usize,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct ACCENTPOLICY {
  pub nAccentState: c_ulong,
  pub nFlags: c_ulong,
  pub nColor: c_ulong,
  pub nAnimationId: c_ulong,
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
      get_function!(LIB_NAME, SetWindowCompositionAttribute);
  }

  *SET_WINDOW_COMPOSITION_ATTRIBUTE
}
