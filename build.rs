extern crate cmake;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn find_and_link_qt5() {
    use pkg_config;

    println!("cargo:rustc-link-lib=dylib=stdc++");
    match pkg_config::find_library("Qt5Core Qt5Gui Qt5Qml Qt5Quick Qt5Svg") {
        Ok(lib) => {
            for p in lib.link_paths {
                println!("cargo:rustc-link-search=native={}", p.display());
            }
            for p in lib.libs {
                println!("cargo:rustc-link-lib=dylib={}", p);
            }
            for p in lib.include_paths {
                println!("cargo:include={}", p.display());
            }
        }
        Err(e) => panic!("Qt5 was not found using pkg-config: {}", e)
    }
}

fn find_and_link_dos(cmake_cfg: &mut cmake::Config) {
    let dst = cmake_cfg.build();
    let path = dst.join("build")
        .join("src")
        .join("3rdparty")
        .join("DOtherSide")
        .join("lib");

    if cfg!(windows) {
        println!("cargo:rustc-link-search=native={}", path.join("Release").display());
    } else {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    println!("cargo:rustc-link-lib=static=DOtherSideStatic");
}

fn find_and_link_de(cmake_cfg: &mut cmake::Config) {
    let dst = cmake_cfg.build();
    let path = dst.join("build")
        .join("src")
        .join("lib")
        .join("DOtherSideExtra");

    if cfg!(windows) {
        println!("cargo:rustc-link-search=native={}", path.join("Release").display());
    } else {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    println!("cargo:rustc-link-lib=static=dothersideextra");
}

#[cfg(debug_assertions)]
fn find_resources(cmake_cfg: &mut cmake::Config) {
    let dst = cmake_cfg.build();
    let path = dst.join("build")
        .join("tests")
        .join("resources");

    if cfg!(windows) {
        println!("cargo:rustc-link-search=native={}", path.join("Release").display());
    } else {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    println!("cargo:rustc-link-lib=static=testresources");
}

fn main() {
    let dos_path = PathBuf::from("src")
        .join("3rdparty")
        .join("DOtherSide")
        .join("CMakeLists.txt");
    if !dos_path.exists() {
        panic!("DOtherSide submodule not checked out. Please run 'git submodule init' followed by 'git submodule update'.");
    }

    let mut cmake_cfg = cmake::Config::new(".");
    if let Ok(gen) = env::var("CMAKE_GENERATOR") {
        cmake_cfg.generator(gen);
    }

    println!("cargo:rustc-link-search=native={}", cmake_cfg.build().join("lib").display());

    #[cfg(debug_assertions)]
    find_resources(&mut cmake_cfg);

    find_and_link_de(&mut cmake_cfg);
    find_and_link_dos(&mut cmake_cfg);
    find_and_link_qt5();
}
