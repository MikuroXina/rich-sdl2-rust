use reqwest::IntoUrl;
use std::{
    env, fs,
    io::{self, Seek, SeekFrom},
    path::{Path, PathBuf},
};

fn main() {
    const SDL2_INSTALL_DIR: &str = "SDL2-2.0.16";

    let root_dir = env::var("OUT_DIR").expect("OUT_DIR not found");
    let root = PathBuf::from(&root_dir);

    #[cfg(unix)]
    {
        use std::process::Command;

        if fs::metadata(root.join(SDL2_INSTALL_DIR).join("build").join(".libs")).is_err() {
            const LINK: &str = "https://libsdl.org/release/SDL2-2.0.16.zip";
            let tmp_file = download_sdl2(LINK, "SDL2-2.0.16.zip");
            extract_zip(tmp_file, &root);

            let _ = Command::new("./configure")
                .current_dir(root.join(SDL2_INSTALL_DIR))
                .output()
                .expect("failed to configure");
            let _ = Command::new("make")
                .arg("-j4")
                .current_dir(root.join(SDL2_INSTALL_DIR))
                .output()
                .expect("failed to make");
        }

        println!(
            "cargo:rustc-link-search={}",
            root.join(SDL2_INSTALL_DIR)
                .join("build")
                .join(".libs")
                .as_path()
                .to_string_lossy()
        );
    }
    #[cfg(windows)]
    {
        const LINK: &str = "https://libsdl.org/release/SDL2-devel-2.0.16-VC.zip";
        let tmp_file = download_sdl2(LINK, "SDL2-2.0.16.tar.gz");
        extract_zip(tmp_file, &root);

        println!(
            "cargo:rustc-link-search={}",
            root.join(SDL2_INSTALL_DIR)
                .join("lib")
                .join("x64")
                .as_path()
                .to_string_lossy()
        );
    }

    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(&format!("-I{}/{}/include", root_dir, SDL2_INSTALL_DIR))
        .allowlist_function("SDL_.*")
        .allowlist_type("SDL_.*")
        .allowlist_var("SDL_.*")
        .generate_comments(false)
        .prepend_enum_name(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("bindgen builder was invalid");

    bindings
        .write_to_file(root.join("bind.rs"))
        .expect("writing `bind.rs` failed");
}

fn download_sdl2(link: impl IntoUrl + Clone, file_name: impl AsRef<Path>) -> fs::File {
    let tmp_dir = env::temp_dir();
    let mut tmp_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(tmp_dir.join(file_name))
        .expect("Failed to create temporary file");

    let mut retry = 3;
    let mut got = loop {
        let result = reqwest::blocking::get(link.clone());
        if let Ok(result) = result {
            break result;
        }
        retry -= 1;
        if retry == 0 {
            panic!("invalid link url: {:?}", link.as_str());
        }
    };

    io::copy(&mut got, &mut tmp_file).expect("failed to write to temporary file");
    tmp_file.seek(SeekFrom::Start(0)).expect("failed to seek");
    tmp_file
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
