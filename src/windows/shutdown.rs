use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use windows::{
  core::PCWSTR,
  Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    System::{
      Shutdown::{ShutdownBlockReasonCreate, ShutdownBlockReasonDestroy},
      Threading::SetProcessShutdownParameters,
    },
    UI::WindowsAndMessaging::{
      CallWindowProcW, SetWindowLongPtrW, GWLP_WNDPROC, WM_ENDSESSION, WM_QUERYENDSESSION, WNDPROC,
    },
  },
};

pub static mut MAIN_WINDOW: HWND = HWND(0);
pub static mut PREV_WND_PROC: WNDPROC = None;
pub static mut SHOULD_BLOCK_SHUTDOWN: bool = false;
pub static mut TSFN: Option<ThreadsafeFunction<u32>> = None;

/**
 * implement windows 'shutdown' event for electron
 *
 * ref: https://github.com/paymoapp/electron-shutdown-handler/blob/master/module/WinShutdownHandler.cpp
 */

pub unsafe fn _set_main_window_handle(h_wnd: HWND) -> () {
  MAIN_WINDOW = h_wnd;

  SetProcessShutdownParameters(0x3FF, 0);
}

pub unsafe fn _insert_wnd_proc_hook() -> bool {
  if MAIN_WINDOW.eq(&HWND::default()) {
    return false;
  }

  if let Some(_) = PREV_WND_PROC {
    return false;
  }

  PREV_WND_PROC = Some(std::mem::transmute(SetWindowLongPtrW(
    MAIN_WINDOW,
    GWLP_WNDPROC,
    window_proc as _,
  )));

  true
}

pub unsafe fn _remove_wnd_proc_hook() -> bool {
  if MAIN_WINDOW.eq(&HWND::default()) {
    return false;
  }

  if let Some(proc) = PREV_WND_PROC {
    TSFN = None;
    SetWindowLongPtrW(MAIN_WINDOW, GWLP_WNDPROC, proc as _);
  }

  true
}

pub unsafe fn _acquire_shutdown_block(reason: &str) -> bool {
  if MAIN_WINDOW.eq(&HWND::default()) {
    return false;
  }
  
  let mut _reason: Vec<u16> = reason.encode_utf16().collect();
  _reason.push(0);
  ShutdownBlockReasonCreate(MAIN_WINDOW, PCWSTR(_reason.as_ptr()));
  SHOULD_BLOCK_SHUTDOWN = true;

  true
}

pub unsafe fn _release_shutdown_block() -> bool {
  if MAIN_WINDOW.eq(&HWND::default()) {
    return  false;
  }

  let result = ShutdownBlockReasonDestroy(MAIN_WINDOW);
  SHOULD_BLOCK_SHUTDOWN = false;

  result.into()
}

unsafe extern "system" fn window_proc(
  h_wnd: HWND,
  event: u32,
  w_param: WPARAM,
  l_param: LPARAM,
) -> LRESULT {
  if event == WM_QUERYENDSESSION {
    if let Some(jsfn) = &TSFN {
      jsfn.call(Ok(1), ThreadsafeFunctionCallMode::Blocking);
    }

    if SHOULD_BLOCK_SHUTDOWN {
      return LRESULT(0);
    }

    return LRESULT(1);
  } else if event == WM_ENDSESSION {
    if w_param.0 == 0 {
      return LRESULT(0);
    }
  }

  CallWindowProcW(PREV_WND_PROC, h_wnd, event, w_param, l_param)
}
