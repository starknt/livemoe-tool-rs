use std::ops::Deref;
use winapi::shared::windef::HWND;

pub struct SyncHWND(pub HWND);

impl SyncHWND {
  pub fn hwnd(&self) -> HWND {
    self.0
  }

  pub fn change(&mut self, hwnd: HWND) {
    self.0 = hwnd;
  }

  pub fn is_null(&self) -> bool {
    self.0.is_null()
  }
}

unsafe impl Sync for SyncHWND {}

impl Deref for SyncHWND {
  type Target = HWND;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}


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
