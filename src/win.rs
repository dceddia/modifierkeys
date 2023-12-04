extern crate windows;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT,
};

use crate::ModifierKeys;

fn is_shift_pressed() -> bool {
    unsafe { GetAsyncKeyState(VK_SHIFT.0.into()) as u16 & 0x8000 != 0 }
}

fn is_control_pressed() -> bool {
    unsafe { GetAsyncKeyState(VK_CONTROL.0.into()) as u16 & 0x8000 != 0 }
}

fn is_alt_pressed() -> bool {
    unsafe { GetAsyncKeyState(VK_MENU.0.into()) as u16 & 0x8000 != 0 }
}

fn is_command_pressed() -> bool {
    unsafe {
        GetAsyncKeyState(VK_LWIN.0.into()) as u16 & 0x8000 != 0
            || GetAsyncKeyState(VK_RWIN.0.into()) as u16 & 0x8000 != 0
    }
}

pub fn read_modifier_keys() -> ModifierKeys {
    ModifierKeys {
        shift: is_shift_pressed(),
        control: is_control_pressed(),
        alt: is_alt_pressed(),
        meta: is_command_pressed(),
    }
}
