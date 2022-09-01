import { beforeAll, expect } from 'vitest'
import { test } from 'vitest'

import { hideDesktopIcon, setWindowWorker, restoreWindowWorker, setTaskbarStyle, ACCENT } from '../index'

test('', () => {
  expect(restoreWindowWorker()).toMatchInlineSnapshot('undefined')
  expect(hideDesktopIcon()).toMatchInlineSnapshot('undefined')
  expect(setTaskbarStyle(ACCENT.AccentNormal, 0x0)).toMatchInlineSnapshot('true')
})