import { beforeAll, expect } from 'vitest'
import { test } from 'vitest'

import { hideDesktopIcon, setWindowWorker, restoreWindowWorker, setTaskbarStyle, ACCENT, showDesktopIcon, getSysListViewIconRect, setSystemCursorStyle, restoreSystemCursorStyle } from '../index'

test('', () => {
  expect(restoreWindowWorker()).toMatchInlineSnapshot('undefined')
  expect(showDesktopIcon()).toMatchInlineSnapshot('undefined')
  expect(setTaskbarStyle(ACCENT.AccentNormal, 0x0)).toMatchInlineSnapshot('true')
  expect(getSysListViewIconRect()).toMatchInlineSnapshot(`
    [
      {
        "bottom": 92,
        "left": 5,
        "right": 94,
        "top": 0,
      },
      {
        "bottom": 481,
        "left": 374,
        "right": 379,
        "top": 285,
      },
    ]
  `);

  expect(restoreSystemCursorStyle()).toMatchInlineSnapshot('undefined')

})