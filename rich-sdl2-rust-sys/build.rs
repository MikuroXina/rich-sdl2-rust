use std::{
    env,
    path::{Path, PathBuf},
};

use git2::Repository;
use retry::{delay::Fixed, retry};

#[cfg(not(any(feature = "static", feature = "dynamic")))]
compile_error!(r#"Either feature "static" or "dynamic" must be enabled."#);

#[cfg(all(feature = "static", feature = "dynamic"))]
compile_error!(r#"Feature "static" and "dynamic" cannot coexist."#);

const SDL_VERSION: &str = "2.26.2";
const SDL_TTF_VERSION: &str = "2.20.1";
const SDL_MIXER_VERSION: &str = "2.6.2";
const SDL_IMAGE_VERSION: &str = "2.6.2";
const SDL_NET_VERSION: &str = "2.2.0";

fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let target_os = target.splitn(3, '-').nth(2).unwrap();

    let includes: Vec<_> = include_paths(target_os)
        .map(|path| format!("-I{}", path.display()))
        .collect();
    eprintln!("{includes:?}");

    set_link(target_os);

    set_lib_dir();

    println!("cargo:rerun-if-changed=wrapper.h");

    let mut builder = bindgen::Builder::default();
    {
        builder = builder
            .header("wrapper.h")
            .clang_args(&includes)
            .clang_args(["-D__AVX512VLFP16INTRIN_H", "-D__AVX512FP16INTRIN_H"])
            .allowlist_function("SDL_.*")
            .allowlist_type("SDL_.*")
            .allowlist_var("SDL_.*")
            .generate_comments(false)
            .prepend_enum_name(false)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks));
    }
    if cfg!(feature = "ttf") {
        builder = builder
            .clang_arg("-DRICH_SDL2_RUST_TTF")
            .allowlist_function("TTF_.*")
            .allowlist_type("TTF_.*")
            .allowlist_var("TTF_.*");
    }
    if cfg!(feature = "mixer") {
        builder = builder
            .clang_arg("-DRICH_SDL2_RUST_MIXER")
            .allowlist_function("Mix_.*")
            .allowlist_type("MIX_.*")
            .allowlist_type("Mix_.*")
            .allowlist_var("MIX_.*")
            .allowlist_var("Mix_.*");
    }
    if cfg!(feature = "image") {
        builder = builder
            .clang_arg("-DRICH_SDL2_RUST_IMAGE")
            .allowlist_function("IMG_.*")
            .allowlist_function("Img_.*")
            .allowlist_type("IMG_.*")
            .allowlist_type("Img_.*")
            .allowlist_var("IMG_.*")
            .allowlist_var("Img_.*");
    }
    if cfg!(feature = "net") {
        builder = builder
            .clang_arg("-DRICH_SDL2_RUST_NET")
            .allowlist_function("SDLNet_.*")
            .allowlist_type("SDLNet_.*")
            .allowlist_var("SDLNet_.*")
    }
    let bindings = builder.generate().expect("bindgen builder was invalid");

    let root_dir = env::var("OUT_DIR").expect("OUT_DIR not found");
    let root = PathBuf::from(root_dir);
    bindings
        .write_to_file(root.join("bind.rs"))
        .expect("writing `bind.rs` failed");
}

fn include_paths(target_os: &str) -> impl Iterator<Item = PathBuf> {
    let mut include_paths = vec![];
    if cfg!(feature = "vendor") {
        let root_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not found"));
        let lib_dir = root_dir.join("lib");
        let include_dir = root_dir.join("include");

        // setup vendored
        build_vendor_sdl2(target_os, &include_dir, &lib_dir, &root_dir);
        println!("cargo:rustc-link-search={}", lib_dir.display());
        eprintln!("vendored SDL: {}", root_dir.display());
        include_paths.push(include_dir);
    } else {
        include_paths.extend(
            pkg_config::Config::new()
                .atleast_version(SDL_VERSION)
                .probe("sdl2")
                .into_iter()
                .flat_map(|sdl2| sdl2.include_paths)
                .chain(std::env::var("SDL2_PATH").map(PathBuf::from).into_iter()),
        );
    }
    if cfg!(feature = "ttf") {
        if cfg!(feature = "vendor") {
            let root_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not found"));
            let lib_dir = root_dir.join("lib");

            // setup vendored
            build_vendor_sdl2_ttf(target_os, &root_dir);
            println!("cargo:rustc-link-search={}", lib_dir.display());
            eprintln!("vendored SDL_ttf: {}", root_dir.display());
        } else {
            include_paths.extend(
                pkg_config::Config::new()
                    .atleast_version(SDL_TTF_VERSION)
                    .probe("sdl2_ttf")
                    .into_iter()
                    .flat_map(|sdl2| sdl2.include_paths),
            );
        }
    }
    if cfg!(feature = "mixer") {
        if cfg!(feature = "vendor") {
            let root_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not found"));
            let lib_dir = root_dir.join("lib");

            // setup vendored
            build_vendor_sdl2_mixer(target_os, &root_dir);
            println!("cargo:rustc-link-search={}", lib_dir.display());
            eprintln!("vendored SDL_mixer: {}", root_dir.display());
        } else {
            include_paths.extend(
                pkg_config::Config::new()
                    .atleast_version(SDL_MIXER_VERSION)
                    .probe("sdl2_mixer")
                    .into_iter()
                    .flat_map(|sdl2| sdl2.include_paths),
            );
        }
    }
    if cfg!(feature = "image") {
        if cfg!(feature = "vendor") {
            let root_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not found"));
            let lib_dir = root_dir.join("lib");

            // setup vendored
            build_vendor_sdl2_image(target_os, &root_dir);
            println!("cargo:rustc-link-search={}", lib_dir.display());
            eprintln!("vendored SDL_image: {}", root_dir.display());
        } else {
            include_paths.extend(
                pkg_config::Config::new()
                    .atleast_version(SDL_IMAGE_VERSION)
                    .probe("sdl2_image")
                    .into_iter()
                    .flat_map(|sdl2| sdl2.include_paths),
            );
        }
    }
    if cfg!(feature = "net") {
        if cfg!(feature = "vendor") {
            let root_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not found"));
            let lib_dir = root_dir.join("lib");

            // setup vendored
            build_vendor_sdl2_net(target_os, &root_dir);
            println!("cargo:rustc-link-search={}", lib_dir.display());
            eprintln!("vendored SDL_net: {}", root_dir.display());
        } else {
            include_paths.extend(
                pkg_config::Config::new()
                    .atleast_version(SDL_NET_VERSION)
                    .probe("sdl2_net")
                    .into_iter()
                    .flat_map(|sdl2| sdl2.include_paths),
            );
        }
    }
    include_paths.into_iter()
}

fn build_vendor_sdl2(target_os: &str, include_dir: &Path, lib_dir: &Path, root_dir: &Path) {
    let repo_path = root_dir.join("SDL");
    if repo_path.is_dir() {
        return;
    }

    use std::process::Command;

    eprintln!("SDL cloning into: {}", repo_path.display());
    let url = "https://github.com/libsdl-org/SDL";
    let repo = retry(Fixed::from_millis(2000).take(3), || {
        if std::fs::remove_dir_all(&repo_path).is_ok() {
            eprintln!("cleaned SDL repository dir")
        }
        Repository::clone_recurse(url, &repo_path)
    })
    .expect("failed to clone SDL repository");
    checkout_to_tag(&repo, SDL_VERSION);

    if target_os.contains("windows") {
        let target_platform = if cfg!(target_pointer_width = "64") {
            "Platform=x64"
        } else {
            r#"Platform="Any CPU""#
        };
        assert!(
            Command::new("msbuild")
                .arg(format!("/p:Configuration=Debug,{target_platform}"))
                .arg(repo_path.join("VisualC").join("SDL.sln"))
                .status()
                .expect("failed to build project")
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
        let project_to_use = if cfg!(target_pointer_width = "64") {
            "x64"
        } else {
            "Win32"
        };
        std::fs::create_dir_all(lib_dir).expect("failed to create lib dir");
        for file in std::fs::read_dir(repo_path.join("VisualC").join(project_to_use).join("Debug"))
            .expect("build dir not found")
            .flatten()
        {
            let path = file.path();
            if path.is_file() {
                eprintln!("built library: {}", path.display());
                std::fs::copy(&path, lib_dir.join(path.file_name().unwrap()))
                    .expect("failed to copy built library");
            }
        }
    } else {
        assert!(
            Command::new(repo_path.join("autogen.sh"))
                .current_dir(&repo_path)
                .status()
                .expect("failed to autogen SDL")
                .success(),
            "autogen failed"
        );
        let build_path = repo_path.join("build");
        std::fs::create_dir(&build_path).expect("failed to mkdir build");
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args([
                    format!("-DCMAKE_INSTALL_PREFIX={}", root_dir.display()),
                    "..".to_string(),
                ])
                .status()
                .expect("failed to configure SDL")
                .success(),
            "cmake failed"
        );
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args(["--build", "."])
                .status()
                .expect("failed to build SDL")
                .success(),
            "build failed"
        );
        assert!(
            Command::new("cmake")
                .args(["--install", "."])
                .current_dir(&build_path)
                .status()
                .expect("failed to setup SDL")
                .success(),
            "setup failed"
        );
    }
}

fn build_vendor_sdl2_ttf(target_os: &str, root_dir: &Path) {
    let repo_path = root_dir.join("SDL_ttf");
    if repo_path.is_dir() {
        return;
    }

    use std::process::Command;

    eprintln!("SDL_ttf cloning into: {}", repo_path.display());
    let url = "https://github.com/libsdl-org/SDL_ttf";
    let repo = retry(Fixed::from_millis(2000).take(3), || {
        if std::fs::remove_dir_all(&repo_path).is_ok() {
            eprintln!("cleaned SDL_ttf repository dir")
        }
        Repository::clone_recurse(url, &repo_path)
    })
    .expect("failed to clone SDL_ttf repository");
    checkout_to_tag(&repo, SDL_TTF_VERSION);

    if target_os.contains("windows") {
        let build_path = repo_path.join("build");
        std::fs::create_dir(&build_path).expect("failed to mkdir build");
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args([
                    format!("-DCMAKE_INSTALL_PREFIX={}", root_dir.display()),
                    "..".to_string(),
                ])
                .status()
                .expect("failed to configure SDL_ttf")
                .success(),
            "cmake failed"
        );
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args(["--build", "."])
                .status()
                .expect("failed to build SDL_ttf")
                .success(),
            "build failed"
        );
        for entry in std::fs::read_dir(build_path.join("Debug"))
            .unwrap()
            .flatten()
        {
            // TODO: remove
            eprintln!("{}", entry.path().display());
        }
        std::fs::rename(
            build_path.join("Debug").join("SDL2_ttfd.dll"),
            root_dir.join("lib").join("SDL2_ttf.dll"),
        )
        .expect("failed to move dll");
        std::fs::rename(
            build_path.join("Debug").join("SDL2_ttfd.lib"),
            root_dir.join("lib").join("SDL2_ttf.lib"),
        )
        .expect("failed to move lib");
        std::fs::copy(
            repo_path.join("include").join("SDL_ttf.h"),
            root_dir.join("include").join("SDL2").join("SDL_ttf.h"),
        )
        .expect("failed to copy header");
    } else {
        let build_path = repo_path.join("build");
        std::fs::create_dir(&build_path).expect("failed to mkdir build");
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args([
                    format!("-DCMAKE_INSTALL_PREFIX={}", root_dir.display()),
                    "..".to_string(),
                ])
                .status()
                .expect("failed to configure SDL_ttf")
                .success(),
            "cmake failed"
        );
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args(["--build", "."])
                .status()
                .expect("failed to build SDL_ttf")
                .success(),
            "build failed"
        );
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args(["--install", "."])
                .status()
                .expect("failed to setup SDL_ttf")
                .success(),
            "setup failed"
        );
    }
}

fn build_vendor_sdl2_mixer(target_os: &str, root_dir: &Path) {
    let repo_path = root_dir.join("SDL_mixer");
    if repo_path.is_dir() {
        return;
    }

    use std::process::Command;

    eprintln!("SDL_mixer cloning into: {}", repo_path.display());
    let url = "https://github.com/libsdl-org/SDL_mixer";
    let repo = retry(Fixed::from_millis(2000).take(3), || {
        if std::fs::remove_dir_all(&repo_path).is_ok() {
            eprintln!("cleaned SDL_mixer repository dir")
        }
        Repository::clone_recurse(url, &repo_path)
    })
    .expect("failed to clone SDL_mixer repository");
    checkout_to_tag(&repo, SDL_MIXER_VERSION);
    for mut submodule in repo.submodules().unwrap() {
        submodule
            .update(true, None)
            .expect("failed to update submodule");
    }

    if target_os.contains("windows") {
        let build_path = repo_path.join("build");
        std::fs::create_dir(&build_path).expect("failed to mkdir build");
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args([
                    format!("-DCMAKE_INSTALL_PREFIX={}", root_dir.display()),
                    "..".into(),
                ])
                .status()
                .expect("failed to configure SDL_mixer")
                .success(),
            "cmake failed"
        );
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args(["--build", "."])
                .status()
                .expect("failed to build SDL_mixer")
                .success(),
            "build failed"
        );
        std::fs::rename(
            build_path.join("Debug").join("SDL2_mixerd.dll"),
            root_dir.join("lib").join("SDL2_mixer.dll"),
        )
        .expect("failed to move dll");
        std::fs::rename(
            build_path.join("Debug").join("SDL2_mixerd.lib"),
            root_dir.join("lib").join("SDL2_mixer.lib"),
        )
        .expect("failed to move lib");
        std::fs::copy(
            repo_path.join("include").join("SDL_mixer.h"),
            root_dir.join("include").join("SDL2").join("SDL_mixer.h"),
        )
        .expect("failed to copy header");
    } else {
        let build_path = repo_path.join("build");
        std::fs::create_dir(&build_path).expect("failed to mkdir build");
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args([
                    format!("-DCMAKE_INSTALL_PREFIX={}", root_dir.display()),
                    "-DSDL2MIXER_VENDORED=ON".into(),
                    "-DSDL2MIXER_BUILD_SHARED_LIBS=ON".into(),
                    "..".into(),
                ])
                .status()
                .expect("failed to configure SDL_mixer")
                .success(),
            "cmake failed"
        );
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args(["--build", "."])
                .status()
                .expect("failed to build SDL_mixer")
                .success(),
            "build failed"
        );
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args(["--install", "."])
                .status()
                .expect("failed to setup SDL_mixer")
                .success(),
            "setup failed"
        );
    }
}

fn build_vendor_sdl2_image(target_os: &str, root_dir: &Path) {
    let repo_path = root_dir.join("SDL_image");
    if repo_path.is_dir() {
        return;
    }

    use std::process::Command;

    eprintln!("SDL_image cloning into: {}", repo_path.display());
    let url = "https://github.com/libsdl-org/SDL_image";
    let repo = retry(Fixed::from_millis(2000).take(3), || {
        if std::fs::remove_dir_all(&repo_path).is_ok() {
            eprintln!("cleaned SDL_image repository dir")
        }
        Repository::clone_recurse(url, &repo_path)
    })
    .expect("failed to clone SDL_image repository");
    checkout_to_tag(&repo, SDL_IMAGE_VERSION);
    for mut submodule in repo.submodules().unwrap() {
        submodule
            .update(true, None)
            .expect("failed to update submodule");
    }

    if target_os.contains("windows") {
        let build_path = repo_path.join("build");
        std::fs::create_dir(&build_path).expect("failed to mkdir build");
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args([
                    format!("-DCMAKE_INSTALL_PREFIX={}", root_dir.display()),
                    "..".into(),
                ])
                .status()
                .expect("failed to configure SDL_image")
                .success(),
            "cmake failed"
        );
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args(["--build", "."])
                .status()
                .expect("failed to build SDL_image")
                .success(),
            "build failed"
        );
        std::fs::rename(
            build_path.join("Debug").join("SDL_imaged.dll"),
            root_dir.join("lib").join("SDL2_mixer.dll"),
        )
        .expect("failed to move dll");
        std::fs::rename(
            build_path.join("Debug").join("SDL_imaged.lib"),
            root_dir.join("lib").join("SDL2_mixer.lib"),
        )
        .expect("failed to move lib");
        std::fs::copy(
            repo_path.join("include").join("SDL_image.h"),
            root_dir.join("include").join("SDL2").join("SDL_image.h"),
        )
        .expect("failed to copy header");
    } else {
        let build_path = repo_path.join("build");
        std::fs::create_dir(&build_path).expect("failed to mkdir build");
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args([
                    format!("-DCMAKE_INSTALL_PREFIX={}", root_dir.display()),
                    "-DSDL2IMAGE_VENDORED=ON".into(),
                    "-DSDL2IMAGE_BUILD_SHARED_LIBS=ON".into(),
                    "..".into(),
                ])
                .status()
                .expect("failed to configure SDL_image")
                .success(),
            "cmake failed"
        );
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args(["--build", "."])
                .status()
                .expect("failed to build SDL_image")
                .success(),
            "build failed"
        );
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args(["--install", "."])
                .status()
                .expect("failed to setup SDL_image")
                .success(),
            "setup failed"
        );
    }
}

fn build_vendor_sdl2_net(target_os: &str, root_dir: &Path) {
    let repo_path = root_dir.join("SDL_net");
    if repo_path.is_dir() {
        return;
    }

    use std::process::Command;

    eprintln!("SDL_net cloning into: {}", repo_path.display());
    let url = "https://github.com/libsdl-org/SDL_net";
    let repo = retry(Fixed::from_millis(2000).take(3), || {
        if std::fs::remove_dir_all(&repo_path).is_ok() {
            eprintln!("cleaned SDL_net repository dir")
        }
        Repository::clone_recurse(url, &repo_path)
    })
    .expect("failed to clone SDL_net repository");
    checkout_to_tag(&repo, SDL_NET_VERSION);
    for mut submodule in repo.submodules().unwrap() {
        submodule
            .update(true, None)
            .expect("failed to update submodule");
    }

    if target_os.contains("windows") {
        let build_path = repo_path.join("build");
        std::fs::create_dir(&build_path).expect("failed to mkdir build");
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args([
                    format!("-DCMAKE_INSTALL_PREFIX={}", root_dir.display()),
                    "..".into(),
                ])
                .status()
                .expect("failed to configure SDL_net")
                .success(),
            "cmake failed"
        );
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args(["--build", "."])
                .status()
                .expect("failed to build SDL_net")
                .success(),
            "build failed"
        );
        std::fs::rename(
            build_path.join("Debug").join("SDL_netd.dll"),
            root_dir.join("lib").join("SDL2_mixer.dll"),
        )
        .expect("failed to move dll");
        std::fs::rename(
            build_path.join("Debug").join("SDL_netd.lib"),
            root_dir.join("lib").join("SDL2_mixer.lib"),
        )
        .expect("failed to move lib");
        std::fs::copy(
            repo_path.join("include").join("SDL_net.h"),
            root_dir.join("include").join("SDL2").join("SDL_net.h"),
        )
        .expect("failed to copy header");
    } else {
        let build_path = repo_path.join("build");
        std::fs::create_dir(&build_path).expect("failed to mkdir build");
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args([
                    format!("-DCMAKE_INSTALL_PREFIX={}", root_dir.display()),
                    "-DSDL2IMAGE_VENDORED=ON".into(),
                    "-DSDL2IMAGE_BUILD_SHARED_LIBS=ON".into(),
                    "..".into(),
                ])
                .status()
                .expect("failed to configure SDL_net")
                .success(),
            "cmake failed"
        );
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args(["--build", "."])
                .status()
                .expect("failed to build SDL_net")
                .success(),
            "build failed"
        );
        assert!(
            Command::new("cmake")
                .current_dir(&build_path)
                .args(["--install", "."])
                .status()
                .expect("failed to setup SDL_net")
                .success(),
            "setup failed"
        );
    }
}

fn checkout_to_tag(repo: &Repository, tag: &str) {
    let (obj, reference) = repo
        .revparse_ext(&format!("release-{tag}"))
        .expect("the version tag not found");
    repo.checkout_tree(&obj, None).expect("failed to checkout");
    match reference {
        Some(gref) => repo.set_head(gref.name().unwrap()),
        None => repo.set_head_detached(obj.id()),
    }
    .expect("Failed to set HEAD");
}

fn set_link(target_os: &str) {
    #[cfg(feature = "static")]
    println!("cargo:rustc-link-lib=static=SDL2");
    #[cfg(feature = "dynamic")]
    println!("cargo:rustc-link-lib=dylib=SDL2");
    #[cfg(feature = "ttf")]
    {
        #[cfg(feature = "static")]
        println!("cargo:rustc-link-lib=static=SDL2_ttf");
        #[cfg(feature = "dynamic")]
        println!("cargo:rustc-link-lib=dylib=SDL2_ttf");
    }
    #[cfg(feature = "mixer")]
    {
        #[cfg(feature = "static")]
        println!("cargo:rustc-link-lib=static=SDL2_mixer");
        #[cfg(feature = "dynamic")]
        println!("cargo:rustc-link-lib=dylib=SDL2_mixer");
    }
    #[cfg(feature = "image")]
    {
        #[cfg(feature = "static")]
        println!("cargo:rustc-link-lib=static=SDL2_image");
        #[cfg(feature = "dynamic")]
        println!("cargo:rustc-link-lib=dylib=SDL2_image");
    }

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
    if cfg!(feature = "vendor") {
        return;
    }
    if let Ok(lib_dir) = std::env::var("SDL2_LIB_DIR") {
        println!("cargo:rustc-link-search={lib_dir}");
    }
}
