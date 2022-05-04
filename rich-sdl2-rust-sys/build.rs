use std::{env, path::PathBuf};

fn main() {
    let includes: Vec<_> = include_paths()
        .map(|path| format!("-I{}", path.display()))
        .collect();
    eprintln!("{:?}", includes);

    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let target_os = target.splitn(3, '-').nth(2).unwrap();
    set_link(target_os);

    set_lib_dir();

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

fn include_paths() -> impl Iterator<Item = PathBuf> {
    pkg_config::Config::new()
        .atleast_version("2.0.16")
        .probe("sdl2")
        .into_iter()
        .flat_map(|sdl2| sdl2.include_paths)
        .chain(std::env::var("SDL2_PATH").map(PathBuf::from).into_iter())
}

fn set_link(target_os: &str) {
    #[cfg(feature = "static")]
    println!("cargo:rustc-link-lib=static=SDL2");
    #[cfg(feature = "dynamic")]
    println!("cargo:rustc-link-lib=SDL2");

    if target_os.contains("windows") {
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=imm32");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=oleaut32");
        println!("cargo:rustc-link-lib=version");
        println!("cargo:rustc-link-lib=uuid");
        println!("cargo:rustc-link-lib=dinput8");
        println!("cargo:rustc-link-lib=dxguid");
        println!("cargo:rustc-link-lib=setupapi");
    } else if target_os == "darwin" {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=Carbon");
        println!("cargo:rustc-link-lib=framework=ForceFeedback");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=iconv");
    }
    #[cfg(feature = "vendor")]
    {
        use git2::Repository;
        use std::process::Command;

        let root_dir = env::var("OUT_DIR").expect("OUT_DIR not found");

        // setup vendored
        let url = "https://github.com/libsdl-org/SDL";
        let mut repo_path = PathBuf::from(root_dir);
        repo_path.push("vendor");
        Repository::clone_recurse(url, &repo_path).expect("failed to clone SDL repository");
        let mut configure = repo_path.clone();
        configure.push("configure");
        Command::new(configure)
            .current_dir(&repo_path)
            .spawn()
            .expect("failed to configure SDL");
        Command::new("make")
            .current_dir(&repo_path)
            .spawn()
            .expect("failed to build SDL");
        let mut lib_dir = repo_path.clone();
        lib_dir.push("build");
        lib_dir.push(".libs");
        println!("cargo:rustc-link-search={}", lib_dir.display());
    }
}

fn set_lib_dir() {
    if let Ok(lib_dir) = std::env::var("SDL2_LIB_DIR") {
        println!("cargo:rustc-link-search={}", lib_dir);
    }
}
