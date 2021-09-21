use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    const SDL2_DIR: &str = "SDL2-2.0.16";

    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not found"));

    #[cfg(unix)]
    if fs::metadata(root.join(SDL2_DIR).join("build").join(".libs")).is_err() {
        let tmp_dir = env::temp_dir();
        eprintln!("{:?}", tmp_dir);
        let mut tmp_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(tmp_dir.join("SDL2-2.0.16.zip"))
            .expect("Failed to create temporary file");

        const LINK: &str = "https://libsdl.org/release/SDL2-2.0.16.zip";
        let mut got = reqwest::blocking::get(LINK).expect("LINK url is invalid");
        io::copy(&mut got, &mut tmp_file).expect("failed to write to temporary file");
        extract_zip(tmp_file, &root);

        let _ = Command::new("./configure")
            .current_dir(SDL2_DIR)
            .output()
            .expect("failed to configure");
        let _ = Command::new("make")
            .arg("-j4")
            .current_dir(SDL2_DIR)
            .output()
            .expect("failed to make");
    }
    #[cfg(windows)]
    {
        const LINK: &str = "https://libsdl.org/release/SDL2-devel-2.0.16-VC.zip";
        todo!()
    }

    println!("cargo:rustc-link-lib=SDL2");
    println!(
        "cargo:rustc-link-search={}",
        root.join(SDL2_DIR)
            .join("build")
            .join(".libs")
            .as_path()
            .to_string_lossy()
    );

    let bindings = bindgen::Builder::default()
        .header_contents(
            "wrapper.h",
            &format!(
                r#"
#define SDL_MAIN_HANDLED
#include "{0}/include/SDL.h"
#include "{0}/include/SDL_vulkan.h"
"#,
                SDL2_DIR
            ),
        )
        .allowlist_function("SDL_.*")
        .allowlist_type("SDL_.*")
        .allowlist_var("SDL_.*")
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

fn extract_zip(file: fs::File, dst: &Path) {
    let mut archive = zip::ZipArchive::new(file).expect("got must be a zip archive");
    for idx in 0..archive.len() {
        let mut file = archive.by_index(idx).unwrap();
        let out_path = match file.enclosed_name() {
            Some(path) => dst.join(path),
            None => continue,
        };
        if file.name().ends_with('/') {
            fs::create_dir_all(&out_path).expect("failed to create directory");
        } else {
            if let Some(parent) = out_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(&parent).expect("failed to create directory");
                }
            }
            let mut out_file = fs::File::create(&out_path).expect("failed to create file");
            io::copy(&mut file, &mut out_file).expect("failed to write to file");
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode))
                    .expect("failed to set permissions");
            }
        }
    }
}
