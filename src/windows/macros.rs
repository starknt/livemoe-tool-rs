#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use winapi::{shared::minwindef::FARPROC, um::libloaderapi::{ GetProcAddress, LoadLibraryW }};

#[macro_export]
macro_rules! TEXT {
    ($x:expr) => {
        {
            use std::ffi::OsStr;
            use std::iter::once;
            use std::os::windows::ffi::OsStrExt;

            let wide: Vec<u16> = OsStr::new($x).encode_wide().chain(once(0)).collect();

            wide.as_ptr()
        }
    };
}

pub fn get_function_impl(library: &str, function: &str) -> Option<FARPROC> {
  assert_eq!(library.chars().last(), Some('\0'));
  assert_eq!(function.chars().last(), Some('\0'));

  let module = unsafe { LoadLibraryW(TEXT!(library)) };
  if module.is_null() {
    return None;
  }
  Some(unsafe { GetProcAddress(module, function.as_ptr() as *const i8) })
}

#[macro_export]
macro_rules! get_function {
  ($lib:expr, $func:ident) => {
    get_function_impl(concat!($lib, '\0'), concat!(stringify!($func), '\0')).map(|f| unsafe {
      std::mem::transmute::<FARPROC, $func>(f)
    })
  };
}
