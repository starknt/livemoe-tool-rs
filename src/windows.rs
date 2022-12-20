mod internal;
mod macros;
mod user32;
mod shutdown;

use self::internal::*;
use self::shutdown::*;
use crate::common::{Alignment, InternalCursorResourceCollection, TaskbarState, ACCENT, RECT};
use std::isize;
use napi::JsFunction;
use windows::Win32::Foundation::{CloseHandle, BOOL, HWND, LPARAM, RECT as WIN_RECT, WPARAM};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Memory::{
  VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RELEASE, PAGE_READWRITE,
};
use windows::Win32::System::Threading::{
  OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
};
use windows::Win32::UI::Controls::{LVM_GETITEMCOUNT, LVM_GETITEMRECT};
use windows::Win32::UI::Shell::{
  SHAppBarMessage, SHQueryUserNotificationState, ABM_GETTASKBARPOS, APPBARDATA,
};
use windows::Win32::UI::WindowsAndMessaging::{
  GetForegroundWindow, GetWindowThreadProcessId, SendMessageW, SetParent, ShowWindow,
  SystemParametersInfoW, SPIF_SENDWININICHANGE, SPI_SETCURSORS, SW_HIDE, SW_SHOW,
};

pub fn set_window_worker(h_wnd: isize) {
  unsafe {
    if WORKER_WINDOW_HANDLER.0 == 0 {
      create_worker_window();
      find_worker_window();

      if WORKER_WINDOW_HANDLER.0 == 0 {
        WORKER_WINDOW_HANDLER = find_progman_window();
      }
    }

    SetParent(HWND(h_wnd), WORKER_WINDOW_HANDLER);
    ShowWindow(WORKER_WINDOW_HANDLER, SW_SHOW);
  }
}

pub fn restore_window_worker() {
  unsafe {
    if WORKER_WINDOW_HANDLER.0 == 0 {
      find_worker_window();
    }

    ShowWindow(WORKER_WINDOW_HANDLER, SW_HIDE);
  }
}

pub fn show_desktop_icon() {
  unsafe {
    if FOLDER_VIEW_WINDOW_HANDLER.0 == 0 {
      FOLDER_VIEW_WINDOW_HANDLER = find_sys_folder_view_window();
    }

    if FOLDER_VIEW_WINDOW_HANDLER.0 != 0 {
      ShowWindow(FOLDER_VIEW_WINDOW_HANDLER, SW_SHOW);
    }
  }
}

pub fn hide_desktop_icon() {
  unsafe {
    if FOLDER_VIEW_WINDOW_HANDLER.0 == 0 {
      FOLDER_VIEW_WINDOW_HANDLER = find_sys_folder_view_window();
    }

    if FOLDER_VIEW_WINDOW_HANDLER.0 != 0 {
      ShowWindow(FOLDER_VIEW_WINDOW_HANDLER, SW_HIDE);
    }
  }
}

pub fn show_shell_window() {
  unsafe {
    let shell = find_tray_shell_window();

    if shell.0 != 0 {
      ShowWindow(shell, SW_SHOW);
    }
  }
}

pub fn hide_shell_window() {
  unsafe {
    let shell = find_tray_shell_window();

    if shell.0 != 0 {
      ShowWindow(shell, SW_HIDE);
    }
  }
}

pub fn show_peek_window() {
  let peek = find_peek_window();

  if peek.0 != 0 {
    unsafe {
      ShowWindow(peek, SW_SHOW);
    }
  }
}

pub fn hide_peek_window() {
  let peek = find_peek_window();

  if peek.0 != 0 {
    unsafe {
      ShowWindow(peek, SW_HIDE);
    }
  }
}

pub fn set_taskbar_style(accept: ACCENT, color: u32) -> bool {
  let taskbar = find_tray_shell_window();

  if taskbar.0 == 0 {
    return false;
  }

  set_taskbar_window_blur(taskbar, accept, color)
}

pub fn restore_taskbar_style() -> bool {
  let taskbar = find_tray_shell_window();

  if taskbar.eq(&HWND::default()) {
    return false;
  }

  set_taskbar_window_blur(taskbar, ACCENT::AccentNormal, 0x0)
}

pub fn query_user_state() -> i32 {
  unsafe {
    match SHQueryUserNotificationState() {
      Ok(state) => return state.0,
      Err(_) => return 0,
    }
  }
}

pub fn get_sys_list_view_icon_rect() -> Vec<RECT> {
  let fold_view = find_sys_folder_view_window();
  let mut icon_collection: Vec<RECT> = vec![];

  if fold_view.ne(&HWND::default()) {
    unsafe {
      let n_max_item = SendMessageW(fold_view, LVM_GETITEMCOUNT, WPARAM(0), LPARAM(0));
      let mut pid: u32 = 0;
      GetWindowThreadProcessId(fold_view, Some(&mut pid));
      let result = OpenProcess(
        PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_QUERY_INFORMATION,
        BOOL(0),
        pid,
      );

      if let Ok(handle) = result {
        let prc = VirtualAllocEx(
          handle,
          None,
          std::mem::size_of::<RECT>(),
          MEM_COMMIT,
          PAGE_READWRITE,
        );

        if !prc.is_null() {
          let mut i: usize = 0;
          let mut num_read: usize = 0;
          while i < n_max_item.0 as usize {
            let mut rect = RECT {
              left: 0,
              top: 0,
              right: 0,
              bottom: 0,
            };

            WriteProcessMemory(
              handle,
              prc,
              &mut rect as *mut _ as _,
              std::mem::size_of::<RECT>(),
              None,
            );
            SendMessageW(fold_view, LVM_GETITEMRECT, WPARAM(i), LPARAM(prc as isize));
            let r = ReadProcessMemory(
              handle,
              prc,
              &mut rect as *mut _ as _,
              std::mem::size_of::<RECT>(),
              Some(&mut num_read as *mut usize),
            );

            if r.as_bool() {
              icon_collection.push(rect);
            }

            i += 1;
          }
          VirtualFreeEx(handle, prc, 0, MEM_RELEASE);
        }

        CloseHandle(handle);
      }
    }
  }

  icon_collection
}

pub fn get_sys_taskbar_state() -> TaskbarState {
  let mut data: APPBARDATA = APPBARDATA {
    cbSize: std::mem::size_of::<APPBARDATA>() as u32,
    hWnd: HWND::default(),
    uCallbackMessage: 0,
    uEdge: 0,
    rc: WIN_RECT {
      top: 0,
      left: 0,
      right: 0,
      bottom: 0,
    },
    lParam: LPARAM(0),
  };

  let mut state: TaskbarState = TaskbarState::new();

  unsafe {
    SHAppBarMessage(ABM_GETTASKBARPOS, &mut data);

    match data.uEdge {
      0 => {
        state.alignment = Alignment::Left;
      }
      2 => {
        state.alignment = Alignment::Right;
      }
      3 => {
        state.alignment = Alignment::Bottom;
      }
      1 => {
        state.alignment = Alignment::Top;
      }
      _ => {
        state.alignment = Alignment::Bottom;
      }
    }

    state.rc = RECT {
      left: data.rc.left,
      top: data.rc.top,
      right: data.rc.right,
      bottom: data.rc.bottom,
    };

    state
  }
}

pub fn set_system_cursor_style(resource: InternalCursorResourceCollection) {
  restore_system_cursor_style();

  internal_set_cursor_style(resource.app_starting, SystemCursorId::AppStarting);
  internal_set_cursor_style(resource.arrow, SystemCursorId::Arrow);
  internal_set_cursor_style(resource.cross, SystemCursorId::Cross);
  internal_set_cursor_style(resource.hand, SystemCursorId::Hand);
  internal_set_cursor_style(resource.i_beam, SystemCursorId::IBeam);
  internal_set_cursor_style(resource.no, SystemCursorId::No);
  internal_set_cursor_style(resource.size, SystemCursorId::Size);
  internal_set_cursor_style(resource.size_all, SystemCursorId::SizeAll);
  internal_set_cursor_style(resource.size_nesw, SystemCursorId::SizeNESW);
  internal_set_cursor_style(resource.size_ns, SystemCursorId::SizeNS);
  internal_set_cursor_style(resource.size_nwse, SystemCursorId::SizeNWSE);
  internal_set_cursor_style(resource.size_we, SystemCursorId::SizeWE);
  internal_set_cursor_style(resource.up_arrow, SystemCursorId::Up);
  internal_set_cursor_style(resource.wait, SystemCursorId::Wait);
  internal_set_cursor_style(resource.help, SystemCursorId::Help);
}

pub fn restore_system_cursor_style() {
  unsafe {
    SystemParametersInfoW(SPI_SETCURSORS, 0, None, SPIF_SENDWININICHANGE);
  }
}

pub fn is_in_desktop_window() -> bool {
  unsafe {
    if WORKER_WINDOW_HANDLER.eq(&HWND::default()) || __WORKER_WINDOW_HANDLER.eq(&HWND::default()) {
      create_worker_window();
      find_worker_window();
    }

    println!("{:?}, {:?}", WORKER_WINDOW_HANDLER, __WORKER_WINDOW_HANDLER);
    println!("{:?}", GetForegroundWindow());

    GetForegroundWindow().eq(&WORKER_WINDOW_HANDLER)
      || GetForegroundWindow().eq(&__WORKER_WINDOW_HANDLER)
  }
}


/**
 * implement windows 'shutdown' event for electron
 *
 * ref: https://github.com/paymoapp/electron-shutdown-handler/blob/master/module/WinShutdownHandler.cpp
 */

pub fn set_main_window_handle(h_wnd: u64) -> bool {
  unsafe {
    _set_main_window_handle(HWND(h_wnd as _))
  }

  true
}

pub fn insert_wnd_proc_hook(callback: JsFunction) -> bool {
  unsafe {
    if let Ok(tsfn) = callback.create_threadsafe_function(0, |ctx| {
      ctx.env.create_uint32(ctx.value + 1).map(|v| vec![v])
    }) {
      TSFN = Some(tsfn)
    }

    _insert_wnd_proc_hook()
  }
}

pub fn remove_wnd_proc_hook() -> bool {
  unsafe {
    _remove_wnd_proc_hook()
  }
}

pub fn acquire_shutdown_block(reason: &str) -> bool {
  unsafe {
    _acquire_shutdown_block(reason)
  }
}

pub fn release_shutdown_block() -> bool {
  unsafe {
    _release_shutdown_block()
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test_fn() {
    assert_eq!(HWND::default(), HWND(0));
  }

  #[test]
  fn test_find_window() {
    assert_eq!(find_peek_window().eq(&HWND::default()), false);
    assert_eq!(find_tray_shell_window().eq(&HWND::default()), false);
    assert_eq!(find_progman_window().eq(&HWND::default()), false);
    assert_eq!(find_sys_folder_view_window().eq(&HWND::default()), false);
  }

  #[test]
  fn test_user_state() {
    println!("{}", query_user_state());

    assert!(query_user_state() > 0);
  }

  #[test]
  fn test_set_taskbar_style() {
    set_taskbar_style(ACCENT::AccentEnableBlurbehind, 0x041eff04);

    std::thread::sleep(std::time::Duration::from_millis(1000 * 10));

    restore_taskbar_style();
  }

  #[test]
  fn test_restore_taskbar_style() {
    restore_taskbar_style();
  }

  #[test]
  fn test_get_sys_folder_view() {
    get_sys_list_view_icon_rect();
  }

  #[test]
  fn test_get_sys_taskbar_state() {
    println!("{:#?}", get_sys_taskbar_state());
  }

  #[test]
  fn test_set_system_cursor_style() {
    // set_system_cursor_style();
  }

  #[test]
  fn test_restore_system_cursor_style() {
    restore_system_cursor_style();
  }

  #[test]
  fn test_is_in_desktop_window() {
    loop {
      println!("{}", is_in_desktop_window());
      std::thread::sleep(std::time::Duration::from_millis(1000));
    }
  }
}
