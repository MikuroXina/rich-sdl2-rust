fn main() {
    use std::env;
    use std::path::PathBuf;

    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_function("SDL_.*")
        .whitelist_type("SDL_.*")
        .whitelist_var("SDL_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .raw_line("//! Rust FFI to `SDL2/SDL.h`")
        .raw_line("")
        .raw_line(r"#![allow(warnings)]")
        .generate()
        .expect("bindgen builder was invalid");

    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not found"));
    bindings
        .write_to_file(root.join("src/bind.rs"))
        .expect("`src` directory not found");
}
