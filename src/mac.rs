use crate::ModifierKeys;

#[link(name = "Carbon", kind = "framework")]
extern "C" {
    fn is_shift_pressed() -> bool;
    fn is_control_pressed() -> bool;
    fn is_command_pressed() -> bool;
    fn is_option_pressed() -> bool;
}

pub fn read_modifier_keys() -> ModifierKeys {
    ModifierKeys {
        shift: unsafe { is_shift_pressed() },
        control: unsafe { is_control_pressed() },
        alt: unsafe { is_option_pressed() },
        meta: unsafe { is_command_pressed() },
    }
}
