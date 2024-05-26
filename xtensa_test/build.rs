extern crate bindgen;

use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let esp_idf_path = "/Users/tokumei/.rustup/toolchains/esp/xtensa-esp-elf/esp-13.2.0_20230928/xtensa-esp-elf/xtensa-esp-elf/include";

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I.")
        .clang_arg(format!("-I{}", esp_idf_path))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
