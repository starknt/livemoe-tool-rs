import { beforeAll, expect } from 'vitest'
import { test } from 'vitest'

import { hideDesktopIcon, setWindowWorker, restoreWindowWorker } from '../index'

test('', () => {
  expect(restoreWindowWorker()).toMatchSnapshot()
  expect(hideDesktopIcon()).toMatchInlineSnapshot('undefined')
})