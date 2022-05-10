use std::{env, path::PathBuf};

#[cfg(not(any(feature = "static", feature = "dynamic", feature = "vendor")))]
compile_error!(r#"Either feature "static", "dynamic" or "bar" must be enabled."#);

#[cfg(all(feature = "static", feature = "dynamic"))]
compile_error!(r#"Feature "static" and "dynamic" cannot coexist."#);

fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let target_os = target.splitn(3, '-').nth(2).unwrap();

    let includes: Vec<_> = include_paths(target_os)
        .map(|path| format!("-I{}", path.display()))
        .collect();
    eprintln!("{:?}", includes);

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

fn include_paths(target_os: &str) -> impl Iterator<Item = PathBuf> {
    let vendor_include = if cfg!(feature = "vendor") {
        use git2::Repository;
        use std::process::Command;

        let root_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not found"));
        let lib_dir = root_dir.join("lib");
        let include_dir = root_dir.join("include");

        // setup vendored
        let url = "https://github.com/libsdl-org/SDL";
        let repo_path = root_dir.join("SDL");
        eprintln!("SDL cloning into: {}", repo_path.display());
        let _ = std::fs::create_dir_all(&repo_path);
        if std::fs::remove_dir_all(&repo_path).is_ok() {
            eprintln!("cleaned SDL repository dir")
        }
        Repository::clone_recurse(url, &repo_path).expect("failed to clone SDL repository");
        if target_os.contains("windows") {
            let build = Command::new("msbuild")
                .current_dir(&repo_path)
                .arg(repo_path.join("VisualC"))
                .arg("/p:Configuration=Debug")
                .spawn()
                .expect("failed to build project");
            assert!(
                build
                    .wait_with_output()
                    .expect("build process stopped")
                    .status
                    .success(),
                "build failed"
            );
            let include_install_dir = include_dir.join("SDL2");
            std::fs::create_dir_all(&include_install_dir).expect("failed to create lib dir");
            for file in std::fs::read_dir(repo_path.join("include"))
                .expect("headers not found in repo")
                .flatten()
            {
                let path = file.path();
                if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("h")) {
                    std::fs::copy(&path, include_install_dir.join(path.file_name().unwrap()))
                        .expect("failed to copy header file");
                }
            }
            std::fs::create_dir_all(&lib_dir).expect("failed to create lib dir");
            for file in std::fs::read_dir(repo_path.join("VisualC").join("Win32").join("Debug"))
                .expect("build dir not found")
                .flatten()
            {
                let path = file.path();
                if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("lib")) {
                    std::fs::copy(&path, lib_dir.join(path.file_name().unwrap()))
                        .expect("failed to copy built library");
                }
            }
        } else {
            let build_path = repo_path.join("build");
            std::fs::create_dir(&build_path).expect("failed to mkdir build");
            let cmake = Command::new("cmake")
                .current_dir(&build_path)
                .args([
                    format!("-DCMAKE_INSTALL_PREFIX={}", root_dir.display()),
                    "..".to_string(),
                ])
                .spawn()
                .expect("failed to configure SDL");
            assert!(
                cmake
                    .wait_with_output()
                    .expect("cmake process stopped")
                    .status
                    .success(),
                "cmake failed"
            );
            let build = Command::new("make")
                .current_dir(&build_path)
                .spawn()
                .expect("failed to build SDL");
            assert!(
                build
                    .wait_with_output()
                    .expect("build process stopped")
                    .status
                    .success(),
                "build failed"
            );
            let setup = Command::new("make")
                .arg("install")
                .current_dir(&build_path)
                .spawn()
                .expect("failed to setup SDL");
            assert!(
                setup
                    .wait_with_output()
                    .expect("setup process stopped")
                    .status
                    .success(),
                "setup failed"
            );
        }
        println!("cargo:rustc-link-search={}", lib_dir.display());
        eprintln!("vendored SDL: {}", root_dir.display());
        vec![include_dir]
    } else {
        vec![]
    };
    pkg_config::Config::new()
        .atleast_version("2.0.16")
        .probe("sdl2")
        .into_iter()
        .flat_map(|sdl2| sdl2.include_paths)
        .chain(std::env::var("SDL2_PATH").map(PathBuf::from).into_iter())
        .chain(vendor_include.into_iter())
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
}

fn set_lib_dir() {
    if let Ok(lib_dir) = std::env::var("SDL2_LIB_DIR") {
        println!("cargo:rustc-link-search={}", lib_dir);
    }
}
