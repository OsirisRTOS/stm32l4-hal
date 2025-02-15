extern crate cbindgen;

fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut config: cbindgen::Config = Default::default();

    config.no_includes = true;
    config.includes = vec!["stdint.h".to_string(), "stdbool.h".to_string(), "stdarg.h".to_string()];

    cbindgen::Builder::new()
        .with_config(config)
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("HAL_H")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/hal/lib.h");
}
