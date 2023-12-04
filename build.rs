use cc::Build;

fn main() {
    if cfg!(target_os = "macos") {
        Build::new().file("src/mac.c").compile("modifierkeys");

        println!("cargo:rustc-link-lib=static={}", "modifierkeys");
    }
}
