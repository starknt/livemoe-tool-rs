mod macros;
mod swca;
mod user32;
mod windef;

use crate::windows::user32::get_set_window_composition_attribute_func;
use std::isize;
use std::mem::{size_of_val, transmute};
use std::ptr::null_mut;
use winapi::shared::basetsd::PDWORD_PTR;
use winapi::shared::windef::HWND;
use winapi::um::dwmapi::{DwmSetWindowAttribute, DWMWINDOWATTRIBUTE};
use winapi::um::shellapi::{SHQueryUserNotificationState, QUERY_USER_NOTIFICATION_STATE};
use winapi::um::winuser::{
  EnumWindows, FindWindowExW, FindWindowW, GetShellWindow, SendMessageTimeoutW, SetParent,
  ShowWindow, SMTO_NORMAL, SW_HIDE, SW_SHOW,
};

use self::swca::{AccentPolicy, WindowCompositionAttribute, ACCENT, WinCompositionData};
use self::user32::WINDOWCOMPOSITIONATTRIBDATA;
use self::windef::SyncHWND;
use super::TEXT;

static mut WORKERW: SyncHWND = SyncHWND(null_mut());
static mut DEF_VIEW: SyncHWND = SyncHWND(null_mut());
static mut __WORKERW: SyncHWND = SyncHWND(null_mut());
static mut FOLD_VIEW: SyncHWND = SyncHWND(null_mut());

fn find_progman_window() -> HWND {
  unsafe { FindWindowW(TEXT!("Progman"), TEXT!("Program Manager")) }
}

fn find_tray_shell_window() -> HWND {
  unsafe { FindWindowW(TEXT!("Shell_TrayWnd"), TEXT!("")) }
}

fn find_peek_window() -> HWND {
  unsafe {
    let tray = find_tray_shell_window();
    let notify = FindWindowExW(tray, null_mut(), TEXT!("TrayNotifyWnd"), TEXT!(""));
    FindWindowExW(
      notify,
      null_mut(),
      TEXT!("TrayShowDesktopButtonWClass"),
      TEXT!(""),
    )
  }
}

fn create_worker_window() {
  unsafe {
    let result: PDWORD_PTR = 0 as *mut usize;
    let progman = find_progman_window();
    SendMessageTimeoutW(progman, 0x052C, 0xD, 0x1, SMTO_NORMAL, 1000, result);
  }
}

unsafe extern "system" fn enum_windows_proc(h_wnd: HWND, _: isize) -> i32 {
  let def_view = FindWindowExW(h_wnd, null_mut(), TEXT!("SHELLDLL_DefView"), TEXT!(""));

  if !def_view.is_null() {
    DEF_VIEW.0 = def_view;
    __WORKERW.0 = h_wnd;
    FOLD_VIEW.0 = FindWindowExW(
      DEF_VIEW.0,
      null_mut(),
      TEXT!("SysListView32"),
      TEXT!("FolderView"),
    );
    WORKERW.0 = FindWindowExW(null_mut(), h_wnd, TEXT!("WorkerW"), TEXT!(""));
    return 0;
  }

  1
}

fn find_worker_window() -> HWND {
  unsafe {
    if WORKERW.0.is_null() {
      EnumWindows(Some(enum_windows_proc), 0 as isize);
    }

    WORKERW.0
  }
}

fn find_sys_folder_view_window() -> HWND {
  let mut u_find_count: u32 = 0;

  unsafe {
    let mut h_sys_list_view32_wnd: HWND = null_mut();

    while h_sys_list_view32_wnd.is_null() && u_find_count < 10 {
      let mut h_parent_wnd: HWND = GetShellWindow();
      let mut h_shell_dll_wnd = FindWindowExW(
        h_parent_wnd,
        null_mut(),
        TEXT!("SHELLDLL_DefView"),
        null_mut(),
      );
      h_sys_list_view32_wnd = FindWindowExW(
        h_shell_dll_wnd,
        null_mut(),
        TEXT!("SysListView32"),
        TEXT!("FolderView"),
      );

      if h_sys_list_view32_wnd.is_null() {
        h_parent_wnd = FindWindowExW(null_mut(), null_mut(), TEXT!("WorkerW"), TEXT!(""));

        while h_shell_dll_wnd.is_null() && !h_parent_wnd.is_null() {
          h_shell_dll_wnd = FindWindowExW(
            h_parent_wnd,
            null_mut(),
            TEXT!("SHELLDLL_DefView"),
            null_mut(),
          );
          h_parent_wnd = FindWindowExW(null_mut(), h_parent_wnd, TEXT!("WorkerW"), TEXT!(""));
        }

        h_sys_list_view32_wnd = FindWindowExW(
          h_shell_dll_wnd,
          null_mut(),
          TEXT!("SysListView32"),
          TEXT!("FolderView"),
        );
      }

      if h_sys_list_view32_wnd.is_null() {
        u_find_count += 1;
      } else {
        break;
      }
    }

    h_sys_list_view32_wnd
  }
}

pub fn set_window_worker(h_wnd: *const usize) {
  unsafe {
    if WORKERW.0.is_null() {
      create_worker_window();
      find_worker_window();

      if WORKERW.0.is_null() {
        WORKERW.0 = find_progman_window();
      }
    }

    SetParent(h_wnd as HWND, WORKERW.0);
    ShowWindow(WORKERW.0, SW_SHOW);
  }
}

pub fn restore_window_worker() {
  unsafe {
    if WORKERW.0.is_null() {
      find_worker_window();
    }

    ShowWindow(WORKERW.0, SW_HIDE);
  }
}

pub fn show_desktop_icon() {
  unsafe {
    if FOLD_VIEW.0.is_null() {
      FOLD_VIEW.0 = find_sys_folder_view_window();
    }

    if !FOLD_VIEW.0.is_null() {
      ShowWindow(FOLD_VIEW.0, SW_SHOW);
    }
  }
}

pub fn hide_desktop_icon() {
  unsafe {
    if FOLD_VIEW.0.is_null() {
      FOLD_VIEW.0 = find_sys_folder_view_window();
    }

    println!("{:p}", find_progman_window());

    if !FOLD_VIEW.0.is_null() {
      ShowWindow(FOLD_VIEW.0, SW_HIDE);
    }
  }
}

pub fn show_shell_window() {
  unsafe {
    let shell = find_tray_shell_window();

    if !shell.is_null() {
      ShowWindow(shell, SW_SHOW);
    }
  }
}

pub fn hide_shell_window() {
  unsafe {
    let shell = find_tray_shell_window();

    if !shell.is_null() {
      ShowWindow(shell, SW_HIDE);
    }
  }
}

pub fn show_peek_window() {
  let peek = find_peek_window();

  if !peek.is_null() {
    unsafe {
      ShowWindow(peek, SW_SHOW);
    }
  }
}

pub fn hide_peek_window() {
  let peek = find_peek_window();

  if !peek.is_null() {
    unsafe {
      ShowWindow(peek, SW_HIDE);
    }
  }
}

pub fn set_taskbar_style(accept: ACCENT, color: u32) -> bool {
  let tray = find_tray_shell_window();

  if tray.is_null() {
    return false;
  }

  let set_window_composition_attribute: _ = if let Some(f) = get_set_window_composition_attribute_func() {
    f
} else {
    return false;
};

  unsafe {
    let data = WINDOWCOMPOSITIONATTRIBDATA {
        Attrib: 19,
        pvData: todo!(),
        cbData: todo!(),
    };
    
  }
}

pub fn restore_taskbar_style() {
  let tray = find_tray_shell_window();

  if !tray.is_null() {
    unsafe {
      let set_taskbar_style = get_set_window_composition_attribute_func();
    }
  }
}

pub fn query_user_state() -> u32 {
  unsafe {
    let mut pquns: QUERY_USER_NOTIFICATION_STATE = 0;

    SHQueryUserNotificationState(&mut pquns);

    pquns
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test_find_window() {
    assert_eq!(find_peek_window().is_null(), false);
    assert_eq!(find_tray_shell_window().is_null(), false);
    assert_eq!(find_progman_window().is_null(), false);
    assert_eq!(find_sys_folder_view_window().is_null(), false);
  }

  #[test]
  fn test_user_state() {
    println!("{}", query_user_state());

    assert!(query_user_state() > 0);
  }

  #[test]
  fn test_set_taskbar_style() {
    set_taskbar_style(ACCENT::AccentEnableTransparentGradient, 0x0);
  }
}