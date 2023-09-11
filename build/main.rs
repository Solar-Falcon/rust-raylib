use std::{env, fs, path::PathBuf};

mod api;
use api::Api;

const RAYLIB_API_PATH: &str = "raylib/parser/output/raylib_api.json";

fn build_raylib() {
    let dest = cmake::Config::new("raylib")
        .define("BUILD_EXAMPLES", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        .profile(if cfg!(debug_assertions) {
            "Debug"
        } else {
            "Release"
        })
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dest.join("lib").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dest.join("lib64").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dest.join("lib32").display()
    );

    if cfg!(windows) {
        println!("cargo:rustc-link-lib=dylib=winmm");
        println!("cargo:rustc-link-lib=dylib=gdi32");
        println!("cargo:rustc-link-lib=dylib=user32");
        println!("cargo:rustc-link-lib=dylib=shell32");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
    } else if cfg!(unix) {
        println!("cargo:rustc-link-search=/usr/local/lib");
        println!("cargo:rustc-link-lib=X11");
    }

    println!("cargo:rustc-link-lib=static=raylib");
}

fn main() {
    build_raylib();

    println!("cargo:rerun-if-changed={}", RAYLIB_API_PATH);

    let api_text = fs::read_to_string(RAYLIB_API_PATH).expect("Unable to read raylib api file");
    let api: Api = serde_json::from_str(&api_text).unwrap();

    let code = api.generate_code();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_path.join("raylib_ffi.rs"), code).expect("Unable to write bindings");
}
