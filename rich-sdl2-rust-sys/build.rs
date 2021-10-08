use std::{env, path::PathBuf};

fn main() {
    let sdl2 = vcpkg::Config::new()
        .emit_includes(true)
        .find_package("sdl2")
        .unwrap();
    eprintln!("{:?}", sdl2.include_paths);
    let includes: Vec<_> = sdl2
        .include_paths
        .into_iter()
        .map(|path| format!("-I{}", path.to_string_lossy()))
        .collect();

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&includes)
        .allowlist_function("SDL_.*")
        .allowlist_type("SDL_.*")
        .allowlist_var("SDL_.*")
        .generate_comments(false)
        .prepend_enum_name(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("bindgen builder was invalid");

    let root_dir = env::var("OUT_DIR").expect("OUT_DIR not found");
    let root = PathBuf::from(root_dir);
    bindings
        .write_to_file(root.join("bind.rs"))
        .expect("writing `bind.rs` failed");
}
