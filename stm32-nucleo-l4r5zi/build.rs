extern crate cbindgen;

fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut config: cbindgen::Config = Default::default();

    config.no_includes = true;
    config.includes = vec!["stdint.h".to_string(), "stdbool.h".to_string(), "stdarg.h".to_string()];

    let bindings = cbindgen::Builder::new()
        .with_config(config)
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("HAL_H")
        .generate();

    match bindings {
        Ok(bindings) => {
            bindings.write_to_file("include/hal/lib.h");
        }
        Err(e) => {
            eprintln!("Error generating bindings: {}", e);
        }
    }
}
