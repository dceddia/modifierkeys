use cc::Build;

fn main() {
    let mut build = Build::new();

    build.file("src/mac.c").compile("modifierkeys");

    println!("cargo:rustc-link-lib=static={}", "modifierkeys");
}
