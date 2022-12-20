/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum UserState {
  QunsNotPresent = 1,
  QunsBusy = 2,
  QunsRunningD3dFullScreen = 3,
  QunsPresentationMode = 4,
  QunsAcceptsNotifications = 5,
  QunsQuietTime = 6,
  QunsApp = 7
}
export interface Rect {
  top: number
  left: number
  right: number
  bottom: number
}
export const enum Alignment {
  Left = 0,
  Top = 1,
  Right = 2,
  Bottom = 3
}
export interface TaskbarState {
  rc: Rect
  alignment: Alignment
}
export const enum ACCENT {
  AccentEnableGradient = 1,
  AccentEnableTransparentgradient = 2,
  AccentEnableBlurbehind = 3,
  AccentEnableFluent = 4,
  AccentNormal = 150
}
export interface Color {
  r: number
  g: number
  b: number
  a: number
}
export interface CursorResourceCollection {
  appStarting: string
  arrow: string
  hand: string
  cross: string
  wait: string
  iBeam: string
  no: string
  size: string
  sizeAll: string
  sizeNesw: string
  sizeNs: string
  sizeNwse: string
  sizeWe: string
  upArrow: string
  help: string
}
export function setWindowWorker(hWnd: number): void
export function restoreWindowWorker(): void
export function showDesktopIcon(): void
export function hideDesktopIcon(): void
export function showShellWindow(): void
export function hideShellWindow(): void
export function showPeekWindow(): void
export function hidePeekWindow(): void
export function queryUserState(): number
export function setTaskbarStyle(accept: ACCENT, color: Color): boolean
export function restoreTaskbarStyle(): boolean
export function getSysListViewIconRect(): Array<Rect>
export function getSysTaskbarState(): TaskbarState
export function setSystemCursorStyle(resource: CursorResourceCollection): void
export function restoreSystemCursorStyle(): void
export function isInDesktopWindow(): boolean
export function setMainWindowHandle(hWnd: bigint): boolean
export function insertWndProcHook(callback: (...args: any[]) => any): boolean
export function removeWndProcHook(): boolean
export function acquireShutdownBlock(reason: string): boolean
export function releaseShutdownBlock(): boolean
