extern crate bindgen;

use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    // Path to the ESP-IDF headers and toolchain headers
    let toolchain_include_path = "/Users/tokumei/.rustup/toolchains/esp/xtensa-esp-elf/esp-13.2.0_20230928/xtensa-esp-elf/xtensa-esp-elf/include";
    let esp_idf_components_path = "/Users/tokumei/Work/esp-idf/components";
    let macos_sdk_include_path =
        "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include";

    // Collect all include paths from ESP-IDF components
    let mut include_paths = vec![
        format!("-I{}", toolchain_include_path),
        format!("-I{}", macos_sdk_include_path), // Include the path to the standard C++ library headers on macOS
    ];

    for entry in fs::read_dir(esp_idf_components_path).expect("Directory not found") {
        let entry = entry.expect("Failed to read directory entry");
        if entry.path().is_dir() {
            let component_path = entry.path();

            // Include the main include directory if it exists
            let include_dir = component_path.join("include");
            if include_dir.exists() {
                include_paths.push(format!("-I{}", include_dir.display()));
            }

            let sub_dirs = [
                // esp
                "esp32",
                "esp32/include",
                // freertos
                "FreeRTOS-Kernel/include",
                "config/include/freertos",
                "config/xtensa/include",
                "FreeRTOS-Kernel/portable/xtensa/include/freertos",
                "esp_additions/include",
                // etc
                "platform_include",
            ];
            for sub_dir in &sub_dirs {
                let specific_dir = component_path.join(sub_dir);
                if specific_dir.exists() {
                    include_paths.push(format!("-I{}", specific_dir.display()));
                }
            }
        }
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I.")
        .clang_args(&include_paths)
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .clang_arg("-DconfigNUMBER_OF_CORES=2")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
