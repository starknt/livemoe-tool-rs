import { expect } from 'vitest'
import { test } from 'vitest'
import { setSystemCursorStyle } from '..'
import { join } from 'path'

const base = `E:/Project/JavaScript/Electron/lm-client/assets/LiveMoeCursorResource/cursors/BLUE ALIEN`.replace("\/", "\\")

test('', () => {
  setSystemCursorStyle({
    appStarting: join(base, 'AppStarting.ani'),
    arrow: join(base, 'Arrow.ani'),
    cross: join(base, 'Arrow.ani'),
    hand: join(base, 'Hand.ani'),
    wait: '',
    iBeam: '',
    no: '',
    size: '',
    sizeAll: '',
    sizeNesw: '',
    sizeNs: '',
    sizeNwse: '',
    sizeWe: '',
    upArrow: '',
    help: ''
  })
})