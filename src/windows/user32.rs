use winapi::{
  shared::{minwindef::{ BOOL, FARPROC }, windef::HWND, ntdef::{ PVOID }, basetsd::SIZE_T},
};
use lazy_static::lazy_static;
use crate::{ get_function };
use super::macros::{ get_function_impl };


#[allow(non_snake_case)]
pub type WINDOWCOMPOSITIONATTRIB = u32;

#[allow(non_snake_case)]
#[repr(C)]
pub struct WINDOWCOMPOSITIONATTRIBDATA {
  Attrib: WINDOWCOMPOSITIONATTRIB,
  pvData: PVOID,
  cbData: SIZE_T,
}

pub type SetWindowCompositionAttribute =
  unsafe extern "system" fn(HWND, *mut WINDOWCOMPOSITIONATTRIBDATA) -> BOOL;

pub fn get_set_window_composition_attribute_func() -> Option<SetWindowCompositionAttribute> {
  unsafe {
    lazy_static! {
        static ref SET_WINDOW_COMPOSITION_ATTRIBUTE: Option<SetWindowCompositionAttribute> =
          get_function!("user32.dll", SetWindowCompositionAttribute);
    }

    *SET_WINDOW_COMPOSITION_ATTRIBUTE

    // if let Some(set_window_composition_attribute) = *SET_WINDOW_COMPOSITION_ATTRIBUTE {
    //     unsafe {
    //         // SetWindowCompositionAttribute needs a bigbool (i32), not bool.
    //         let mut data = WINDOWCOMPOSITIONATTRIBDATA {
    //             Attrib: WCA_USEDARKMODECOLORS,
    //             pvData: &mut is_dark_mode_bigbool as *mut _ as _,
    //             cbData: std::mem::size_of_val(&is_dark_mode_bigbool) as _,
    //         };

    //         let status = set_window_composition_attribute(hwnd, &mut data as *mut _);

    //         status != FALSE
    //     }
    // } else {
    //     false
    // }
  }
}
