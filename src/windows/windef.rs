use winapi::shared::windef::HWND;
use std::ops::Deref;

pub struct SyncHWND(pub HWND);

unsafe impl Sync for SyncHWND {}

impl Deref for SyncHWND {
  type Target = HWND;

    fn deref(&self) -> &Self::Target {
      &self.0
    }
}