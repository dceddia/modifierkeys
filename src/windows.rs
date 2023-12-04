extern crate winapi;

use std::os::raw::c_int;
use winapi::um::winnt::SHORT;
use winapi::um::winuser::GetAsyncKeyState;
use winapi::um::winuser::{VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT};

fn is_shift_pressed() {
    GetAsyncKeyState(VK_SHIFT as c_int) & 0x8000 != 0
}

fn is_control_pressed() {
    GetAsyncKeyState(VK_CONTROL as c_int) & 0x8000 != 0
}

fn is_alt_pressed() {
    GetAsyncKeyState(VK_SHIFT as c_int) & 0x8000 != 0
}

fn is_command_pressed() {
    GetAsyncKeyState(VK_SHIFT as c_int) & 0x8000 != 0
}

pub fn read_modifier_keys() -> ModifierKeys {
    ModifierKeys {
        shift: unsafe { is_shift_pressed() },
        control: unsafe { is_control_pressed() },
        alt: unsafe { is_option_pressed() },
        meta: unsafe { is_command_pressed() },
    }
}
