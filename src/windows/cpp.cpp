#include "c.h"

const user32::pSetWindowCompositionAttribute user32::SetWindowCompositionAttribute =
    reinterpret_cast<pSetWindowCompositionAttribute>(
        GetProcAddress(GetModuleHandle(TEXT("user32.dll")), "SetWindowCompositionAttribute"));

void SetWindowCompositionAttribute() {
  user32::SetWindowCompositionAttribute()
}