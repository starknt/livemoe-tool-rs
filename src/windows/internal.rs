use crate::common::ACCENT;
use crate::windows::user32::{ACCENTPOLICY, WINDOWCOMPOSITIONATTRIBDATA};
use std::path::Path;
use wchar::{wchar_t, wchz};

use windows::core::PCWSTR;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, WPARAM};
use windows::Win32::Graphics::Dwm::{DwmEnableBlurBehindWindow, DWM_BLURBEHIND, DwmGetColorizationColor};
use windows::Win32::UI::WindowsAndMessaging::{
  EnumWindows, FindWindowExW, FindWindowW, GetShellWindow, LoadCursorFromFileW,
  SendMessageTimeoutW, SetSystemCursor, SMTO_NORMAL, SYSTEM_CURSOR_ID,
};

use super::user32::get_set_window_composition_attribute_func;

pub const PROGMAN: &[wchar_t] = wchz!("Progman");
pub const PROGMAN_MANAGER: &[wchar_t] = wchz!("Program Manager");
pub const SHELL_TRAY_WND: &[wchar_t] = wchz!("Shell_TrayWnd");
pub const TRAY_NOTIFY_WND: &[wchar_t] = wchz!("TrayNotifyWnd");
pub const SHELL_DLL_DEF_VIEW: &[wchar_t] = wchz!("SHELLDLL_DefView");
pub const TRAY_SHOW_DESKTOP_BTN: &[wchar_t] = wchz!("TrayShowDesktopButtonWClass");
pub const SYS_LIST_VIEW: &[wchar_t] = wchz!("SysListView32");
pub const FOLDER_VIEW: &[wchar_t] = wchz!("FolderView");
pub const WORKER_W: &[wchar_t] = wchz!("WorkerW");
pub const EMPTY: &[wchar_t] = wchz!("");

pub static mut WORKER_WINDOW_HANDLER: HWND = HWND(0);
pub static mut DEF_VIEW_WINDOW_HANDLER: HWND = HWND(0);
pub static mut __WORKER_WINDOW_HANDLER: HWND = HWND(0);
pub static mut FOLDER_VIEW_WINDOW_HANDLER: HWND = HWND(0);
pub static mut TASK_BAR_COLOR: i64 = -1;

pub enum SystemCursorId {
  AppStarting = 32650,
  Arrow = 32512,
  Hand = 32649,
  Cross = 32515,
  IBeam = 32513,
  No = 32648,
  Size = 32640,
  SizeAll = 32646,
  SizeNESW = 32643,
  SizeNS = 32645,
  SizeNWSE = 32642,
  SizeWE = 32644,
  Up = 32516,
  Wait = 32514,
  Help = 32651,
}

impl Into<u32> for SystemCursorId {
  fn into(self) -> u32 {
    self as u32
  }
}

pub fn find_progman_window() -> HWND {
  unsafe { FindWindowW(PCWSTR(PROGMAN.as_ptr()), PCWSTR(PROGMAN_MANAGER.as_ptr())) }
}

pub fn find_tray_shell_window() -> HWND {
  unsafe { FindWindowW(PCWSTR(SHELL_TRAY_WND.as_ptr()), PCWSTR(EMPTY.as_ptr())) }
}

pub fn find_peek_window() -> HWND {
  unsafe {
    let tray = find_tray_shell_window();
    let notify = FindWindowExW(
      tray,
      HWND::default(),
      PCWSTR(TRAY_NOTIFY_WND.as_ptr()),
      PCWSTR(EMPTY.as_ptr()),
    );
    FindWindowExW(
      notify,
      HWND::default(),
      PCWSTR(TRAY_SHOW_DESKTOP_BTN.as_ptr()),
      PCWSTR(EMPTY.as_ptr()),
    )
  }
}

pub fn create_worker_window() {
  unsafe {
    let progman = find_progman_window();
    SendMessageTimeoutW(
      progman,
      0x052C,
      WPARAM(0xD),
      LPARAM(0x1),
      SMTO_NORMAL,
      1000,
      None,
    );
  }
}

pub unsafe extern "system" fn enum_windows_proc(h_wnd: HWND, _: LPARAM) -> BOOL {
  let def_view = FindWindowExW(
    h_wnd,
    HWND::default(),
    PCWSTR(SHELL_DLL_DEF_VIEW.as_ptr()),
    PCWSTR(EMPTY.as_ptr()),
  );

  if def_view.ne(&HWND::default()) {
    DEF_VIEW_WINDOW_HANDLER = def_view;
    __WORKER_WINDOW_HANDLER = h_wnd;
    FOLDER_VIEW_WINDOW_HANDLER = FindWindowExW(
      DEF_VIEW_WINDOW_HANDLER,
      HWND::default(),
      PCWSTR(SYS_LIST_VIEW.as_ptr()),
      PCWSTR(FOLDER_VIEW.as_ptr()),
    );
    WORKER_WINDOW_HANDLER = FindWindowExW(
      HWND::default(),
      h_wnd,
      PCWSTR(WORKER_W.as_ptr()),
      PCWSTR(EMPTY.as_ptr()),
    );
    return false.into();
  }

  true.into()
}

pub fn find_worker_window() -> HWND {
  unsafe {
    if WORKER_WINDOW_HANDLER.eq(&HWND::default()) {
      EnumWindows(Some(enum_windows_proc), LPARAM(0));
    }

    WORKER_WINDOW_HANDLER
  }
}

pub fn find_sys_folder_view_window() -> HWND {
  let mut u_find_count: u32 = 0;

  unsafe {
    let mut h_sys_list_view32_wnd: HWND = Default::default();

    while h_sys_list_view32_wnd.0 == 0 && u_find_count < 10 {
      let mut h_parent_wnd: HWND = GetShellWindow();
      let mut h_shell_dll_wnd = FindWindowExW(
        h_parent_wnd,
        HWND::default(),
        PCWSTR(SHELL_DLL_DEF_VIEW.as_ptr()),
        PCWSTR(EMPTY.as_ptr()),
      );
      h_sys_list_view32_wnd = FindWindowExW(
        h_shell_dll_wnd,
        HWND::default(),
        PCWSTR(SYS_LIST_VIEW.as_ptr()),
        PCWSTR(FOLDER_VIEW.as_ptr()),
      );

      if h_sys_list_view32_wnd.eq(&HWND::default()) {
        h_parent_wnd = FindWindowExW(
          HWND::default(),
          HWND::default(),
          PCWSTR(WORKER_W.as_ptr()),
          PCWSTR(EMPTY.as_ptr()),
        );

        while h_shell_dll_wnd.eq(&HWND::default()) && h_parent_wnd.ne(&HWND::default()) {
          h_shell_dll_wnd = FindWindowExW(
            h_parent_wnd,
            HWND::default(),
            PCWSTR(SHELL_DLL_DEF_VIEW.as_ptr()),
            PCWSTR(EMPTY.as_ptr()),
          );
          h_parent_wnd = FindWindowExW(
            HWND::default(),
            h_parent_wnd,
            PCWSTR(WORKER_W.as_ptr()),
            PCWSTR(EMPTY.as_ptr()),
          );
        }

        h_sys_list_view32_wnd = FindWindowExW(
          h_shell_dll_wnd,
          HWND::default(),
          PCWSTR(SYS_LIST_VIEW.as_ptr()),
          PCWSTR(FOLDER_VIEW.as_ptr()),
        );
      }

      if h_sys_list_view32_wnd.0 == 0 {
        u_find_count += 1;
      } else {
        break;
      }
    }

    h_sys_list_view32_wnd
  }
}

pub fn set_taskbar_window_blur(taskbar: HWND, accept: ACCENT, mut color: u32) -> bool {
  let set_window_composition_attribute: _ =
    if let Some(f) = get_set_window_composition_attribute_func() {
      f
    } else {
      return false;
    };

  unsafe {
    if TASK_BAR_COLOR == -1 {
      let mut _color: u32 = 0;
      let mut blend: BOOL = false.into();
      if let Ok(()) = DwmGetColorizationColor(&mut _color, &mut blend) {
        TASK_BAR_COLOR = _color.into();
      }
    } else if ACCENT::AccentNormal == accept {
      color = TASK_BAR_COLOR as u32;
    }

    match accept {
      ACCENT::AccentEnableBlurbehind => {
        if let Ok(()) = DwmEnableBlurBehindWindow(
          taskbar,
          &DWM_BLURBEHIND {
            fEnable: true.into(),
            hRgnBlur: windows::Win32::Graphics::Gdi::HRGN(3),
            dwFlags: windows::Win32::Graphics::Dwm::DWM_BB_ENABLE,
            fTransitionOnMaximized: true.into(),
          },
        ) {}
      }
      ACCENT::AccentNormal => {
        if let Ok(()) = DwmEnableBlurBehindWindow(
          taskbar,
          &DWM_BLURBEHIND {
            fEnable: false.into(),
            hRgnBlur: windows::Win32::Graphics::Gdi::HRGN(3),
            dwFlags: windows::Win32::Graphics::Dwm::DWM_BB_ENABLE,
            fTransitionOnMaximized: true.into(),
          },
        ) {}
      }
      _ => {}
    }

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

pub fn internal_set_cursor_style(path: Option<String>, style_id: SystemCursorId) {
  let op_path: Option<String> = if let Some(path) = path {
    if !Path::new(&path).exists() {
      return ();
    }

    Some(path)
  } else {
    None
  };

  unsafe {
    use windows::core::Error;

    let op_h_icon = if let Some(path) = op_path {
      let filepath: Vec<u16> = path.encode_utf16().collect();
      LoadCursorFromFileW(PCWSTR(filepath.as_ptr()))
    } else {
      Err(Error::OK)
    };

    if let Ok(h_icon) = op_h_icon {
      SetSystemCursor(h_icon, SYSTEM_CURSOR_ID(style_id.into()));
    }
  }
}
