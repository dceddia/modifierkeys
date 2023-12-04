#[cfg(target_os = "macos")]
mod mac;

#[cfg(target_os = "macos")]
pub use mac::read_modifier_keys;

#[cfg(target_os = "windows")]
mod win;

#[cfg(target_os = "windows")]
pub use win::read_modifier_keys;

#[derive(Debug)]
pub struct ModifierKeys {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub meta: bool,
}
