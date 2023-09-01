use std::{env, fs, path::PathBuf};

mod api;
use api::Api;

const RAYLIB_API_PATH: &str = "raylib/parser/output/raylib_api.json";

fn build_raylib() {
    let dest = cmake::Config::new("raylib")
        .define("BUILD_EXAMPLES", "OFF")
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
