fn main() {
    use std::env;
    use std::path::PathBuf;

    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not found"));

    println!("cargo:rustc-link-lib=SDL2");
    println!(
        "cargo:rustc-link-search={}",
        root.join("SDL2/build").as_path().to_string_lossy()
    );
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_function("SDL_.*")
        .whitelist_type("SDL_.*")
        .whitelist_var("SDL_.*")
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .raw_line("//! Rust FFI to `SDL2/SDL.h`")
        .raw_line("")
        .raw_line(r"#![allow(warnings)]")
        .generate()
        .expect("bindgen builder was invalid");

    bindings
        .write_to_file(root.join("src/bind.rs"))
        .expect("`src` directory not found");
}
