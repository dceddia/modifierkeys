# Read the Modifier Keys on Mac and Windows from Rust

This crate supplies one public function, `read_modifier_keys`, which returns a struct containing the current state of the modifier keys:

```rust
pub struct ModifierKeys {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub meta: bool,
}
```

On Mac, `alt` corresponds to the Option key, and `meta` corresponds to `Command`.

On Windows, `meta` corresponds to the Windows key.
