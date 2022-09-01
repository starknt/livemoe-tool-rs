use crate::{windows::swca::WINCOMPATTRDATA, TEXT};
use winapi::{
  shared::windef::HWND,
  um::libloaderapi::{GetModuleHandleW, GetProcAddress},
};

pub type SetWindowCompositionAttribute = fn(HWND, WINCOMPATTRDATA);

pub fn get_set_window_composition_attribute_func() -> Box<SetWindowCompositionAttribute> {
  unsafe {
    let handle = GetModuleHandleW(TEXT!("user32.dll"));
    let set_window_composition_attribute = GetProcAddress(
      handle,
      "SetWindowCompositionAttribute".as_ptr() as *const i8,
    );

    Box::new(std::mem::transmute::<
      *const (),
      SetWindowCompositionAttribute,
    >(set_window_composition_attribute as *const ()))
  }
}
