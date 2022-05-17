const Addon = require("bindings")("tools");

module.exports = {
  GetSysListViewPosition: Addon.GetSysListViewPosition,
  GetSysTaskbarState: Addon.GetSysTaskbarState,
  GetSysListViewIconRect: Addon.GetSysListViewIconRect,
  SetWindowInWorkerW: Addon.SetWindowInWorkerW,
  RestoreWorkerW: Addon.RestoreWorkerW,
  CheckAeroEnable: Addon.CheckAeroEnable,
  EnableAero: Addon.EnableAero,
  SetTaskbar: Addon.SetTaskbar,
  RestoreTaskbar: Addon.RestoreTaskbar,
  SetSystemCursorToNode: Addon.SetSystemCursorToNode,
  RestoreSystemCursor: Addon.RestoreSystemCursor,
  ShowDesktopIcon: Addon.ShowDesktopIcon,
  HideDesktopIcon: Addon.HideDesktopIcon,
  ShowShellWindow: Addon.ShowShellWindow,
  HideShellWindow: Addon.HideShellWindow,
  QueryUserState: Addon.QueryUserState,
  IsInDesktop: Addon.IsInDesktop,
};
