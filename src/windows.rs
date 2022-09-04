mod macros;
mod user32;
mod windef;

use self::user32::{ACCENTPOLICY, WINDOWCOMPOSITIONATTRIBDATA};
use self::windef::{SyncHWND, SystemCursorId};
use super::TEXT;
use crate::common::{Alignment, InternalCursorResourceCollection, TaskbarState, ACCENT, RECT};
use crate::windows::user32::get_set_window_composition_attribute_func;
use std::isize;
use std::path::Path;
use std::ptr::null_mut;
use winapi::shared::basetsd::PDWORD_PTR;
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, LPARAM, TRUE};
use winapi::shared::windef::{HWND, RECT as WIN_RECT};
use winapi::um::commctrl::{LVM_GETITEMCOUNT, LVM_GETITEMRECT};
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{ReadProcessMemory, VirtualAllocEx, VirtualFreeEx, WriteProcessMemory};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::shellapi::{
  SHAppBarMessage, SHQueryUserNotificationState, ABM_GETTASKBARPOS, APPBARDATA,
  QUERY_USER_NOTIFICATION_STATE,
};
use winapi::um::winnt::{
  MEM_COMMIT, MEM_RELEASE, PAGE_READWRITE, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION,
  PROCESS_VM_READ, PROCESS_VM_WRITE,
};
use winapi::um::winuser::{
  EnumWindows, FindWindowExW, FindWindowW, GetForegroundWindow, GetShellWindow,
  GetWindowThreadProcessId, LoadCursorFromFileW, SendMessageTimeoutW, SendMessageW, SetParent,
  SetSystemCursor, ShowWindow, SystemParametersInfoW, SMTO_NORMAL, SPIF_SENDWININICHANGE,
  SPI_SETCURSORS, SW_HIDE, SW_SHOW,
};

static mut WORKER_WINDOW_HANDLER: SyncHWND = SyncHWND(null_mut());
static mut DEF_VIEW_WINDOW_HANDLER: SyncHWND = SyncHWND(null_mut());
static mut __WORKER_WINDOW_HANDLER: SyncHWND = SyncHWND(null_mut());
static mut FOLDER_VIEW_WINDOW_HANDLER: SyncHWND = SyncHWND(null_mut());

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

unsafe extern "system" fn enum_windows_proc(h_wnd: HWND, _: isize) -> BOOL {
  let def_view = FindWindowExW(h_wnd, null_mut(), TEXT!("SHELLDLL_DefView"), TEXT!(""));

  if !def_view.is_null() {
    DEF_VIEW_WINDOW_HANDLER.change(def_view);
    __WORKER_WINDOW_HANDLER.change(h_wnd);
    FOLDER_VIEW_WINDOW_HANDLER.change(FindWindowExW(
      DEF_VIEW_WINDOW_HANDLER.hwnd(),
      null_mut(),
      TEXT!("SysListView32"),
      TEXT!("FolderView"),
    ));
    WORKER_WINDOW_HANDLER.change(FindWindowExW(
      null_mut(),
      h_wnd,
      TEXT!("WorkerW"),
      TEXT!(""),
    ));
    return FALSE;
  }

  TRUE
}

fn find_worker_window() -> HWND {
  unsafe {
    if WORKER_WINDOW_HANDLER.is_null() {
      EnumWindows(Some(enum_windows_proc), 0 as isize);
    }

    WORKER_WINDOW_HANDLER.hwnd()
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
    if WORKER_WINDOW_HANDLER.is_null() {
      create_worker_window();
      find_worker_window();

      if WORKER_WINDOW_HANDLER.is_null() {
        WORKER_WINDOW_HANDLER.change(find_progman_window());
      }
    }

    SetParent(h_wnd as HWND, WORKER_WINDOW_HANDLER.0);
    ShowWindow(WORKER_WINDOW_HANDLER.hwnd(), SW_SHOW);
  }
}

pub fn restore_window_worker() {
  unsafe {
    if WORKER_WINDOW_HANDLER.is_null() {
      find_worker_window();
    }

    ShowWindow(WORKER_WINDOW_HANDLER.hwnd(), SW_HIDE);
  }
}

pub fn show_desktop_icon() {
  unsafe {
    if FOLDER_VIEW_WINDOW_HANDLER.is_null() {
      FOLDER_VIEW_WINDOW_HANDLER.change(find_sys_folder_view_window());
    }

    if !FOLDER_VIEW_WINDOW_HANDLER.is_null() {
      ShowWindow(FOLDER_VIEW_WINDOW_HANDLER.hwnd(), SW_SHOW);
    }
  }
}

pub fn hide_desktop_icon() {
  unsafe {
    if FOLDER_VIEW_WINDOW_HANDLER.is_null() {
      FOLDER_VIEW_WINDOW_HANDLER.change(find_sys_folder_view_window());
    }

    if !FOLDER_VIEW_WINDOW_HANDLER.is_null() {
      ShowWindow(FOLDER_VIEW_WINDOW_HANDLER.hwnd(), SW_HIDE);
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

fn set_taskbar_window_blur(taskbar: HWND, accept: ACCENT, color: u32) -> bool {
  let set_window_composition_attribute: _ =
    if let Some(f) = get_set_window_composition_attribute_func() {
      f
    } else {
      return false;
    };

  unsafe {
    let mut policy = ACCENTPOLICY {
      nAccentState: accept.into(),
      nFlags: 2,
      nColor: (color & 0xFF00FF00) + ((color & 0x00FF0000) >> 16) + ((color & 0x000000FF) << 16),
      nAnimationId: 0,
    };

    if policy.nAccentState == ACCENT::AccentEnableFluent.into() && policy.nColor >> 24 == 0x00 {
      // Fluent mode doesn't likes a completely 0 opacity
      policy.nColor = (0x01 << 24) + (policy.nColor & 0x00FFFFFF);
    }

    let mut data = WINDOWCOMPOSITIONATTRIBDATA {
      Attrib: 19,
      pvData: &mut policy as *mut _ as _,
      cbData: std::mem::size_of_val(&policy) as _,
    };

    set_window_composition_attribute(taskbar, &mut data as *mut _);

    true
  }
}

pub fn set_taskbar_style(accept: ACCENT, color: u32) -> bool {
  let taskbar = find_tray_shell_window();

  if taskbar.is_null() {
    return false;
  }

  set_taskbar_window_blur(taskbar, accept, color)
}

pub fn restore_taskbar_style() -> bool {
  let taskbar = find_tray_shell_window();

  if taskbar.is_null() {
    return false;
  }

  set_taskbar_window_blur(taskbar, ACCENT::AccentNormal, 0x0)
}

pub fn query_user_state() -> u32 {
  unsafe {
    let mut pquns: QUERY_USER_NOTIFICATION_STATE = 0;

    SHQueryUserNotificationState(&mut pquns);

    pquns
  }
}

pub fn get_sys_list_view_icon_rect() -> Vec<RECT> {
  let fold_view = find_sys_folder_view_window();
  let mut icon_collection: Vec<RECT> = vec![];

  if !fold_view.is_null() {
    unsafe {
      let n_max_item: usize = SendMessageW(fold_view, LVM_GETITEMCOUNT, 0, 0) as usize;
      let mut pid: DWORD = 0;
      GetWindowThreadProcessId(fold_view, &mut pid);
      let handle = OpenProcess(
        PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_QUERY_INFORMATION,
        FALSE as i32,
        pid,
      );

      if !handle.is_null() {
        let prc = VirtualAllocEx(
          handle,
          null_mut(),
          std::mem::size_of::<RECT>(),
          MEM_COMMIT,
          PAGE_READWRITE,
        );

        if !prc.is_null() {
          let mut i: usize = 0;
          let mut num_read: usize = 0;
          while i < n_max_item {
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
              null_mut(),
            );
            SendMessageW(fold_view, LVM_GETITEMRECT, i, prc as LPARAM);
            let r = ReadProcessMemory(
              handle,
              prc,
              &mut rect as *mut _ as _,
              std::mem::size_of::<RECT>(),
              &mut num_read as *mut usize,
            );

            if r > 0 {
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
    hWnd: null_mut(),
    uCallbackMessage: 0,
    uEdge: 0,
    rc: WIN_RECT {
      top: 0,
      left: 0,
      right: 0,
      bottom: 0,
    },
    lParam: 0,
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

fn internal_set_cursor_style(path: Option<String>, style_id: SystemCursorId) {
  let op_path: Option<String> = if let Some(path) = path {
    if !Path::new(&path).exists() {
      return ();
    }

    Some(path)
  } else {
    None
  };

  unsafe {
    let op_h_icon = if let Some(path) = op_path {
      Some(LoadCursorFromFileW(TEXT!(&path)))
    } else {
      None
    };

    if let Some(h_icon) = op_h_icon {
      SetSystemCursor(h_icon, style_id.into());
    }
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
    SystemParametersInfoW(SPI_SETCURSORS, 0, null_mut(), SPIF_SENDWININICHANGE);
  }
}

pub fn is_in_desktop_window() -> bool {
  unsafe {
    if WORKER_WINDOW_HANDLER.is_null() && __WORKER_WINDOW_HANDLER.is_null() {
      create_worker_window();
      find_worker_window();
    }

    GetForegroundWindow() == WORKER_WINDOW_HANDLER.hwnd()
      || GetForegroundWindow() == __WORKER_WINDOW_HANDLER.hwnd()
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
    set_taskbar_style(ACCENT::AccentEnableFluent, 0x0);
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
