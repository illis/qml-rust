extern crate cmake;
extern crate pkg_config;

use std::env;
use std::env::consts;
use std::path::PathBuf;

fn build_dos(cmake_cfg: &mut cmake::Config) {
    let dst = cmake_cfg.build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());

    if cfg!(windows) {
        println!("cargo:rustc-link-search=native={}", dst.join("build").join("lib").join("Release").display());
    } else {
        println!("cargo:rustc-link-search=native={}", dst.join("build").join("lib").display());
    }

    println!("cargo:rustc-link-lib=static=DOtherSideStatic");
}

fn find_qt5(_: &mut cmake::Config) {
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

fn main() {
    let mut cmake_cfg = cmake::Config::new("DOtherSide");
    let dos_path = PathBuf::from("DOtherSide").join("CMakeLists.txt");

    if !dos_path.exists() {
        panic!("DOtherSide submodule not checked out. Please run 'git submodule init' followed by 'git submodule update'.");
    }

    if let Ok(gen) = env::var("CMAKE_GENERATOR") {
        cmake_cfg.generator(gen);
    }

    find_qt5(&mut cmake_cfg);
    build_dos(&mut cmake_cfg);
}
