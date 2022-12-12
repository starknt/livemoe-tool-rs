#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use wchar::wchar_t;
use windows::{Win32::{System::LibraryLoader::{LoadLibraryW, GetProcAddress}, Foundation::FARPROC}, core::{PCSTR, PCWSTR}};


pub fn get_function_impl(library: &[wchar_t], function: &str) -> Option<FARPROC> {
  let result = unsafe { LoadLibraryW(PCWSTR(library.as_ptr())) };
  if let Some(module) = result.ok() {
    return Some(unsafe { GetProcAddress(module, PCSTR(function.as_ptr())) });
  }

  None
}

#[macro_export]
macro_rules! get_function {
  ($lib:expr, $func:ident) => {
    get_function_impl($lib, concat!(stringify!($func), '\0')).map(|f| unsafe {
      std::mem::transmute::<_, $func>(f)
    })
  };
}
