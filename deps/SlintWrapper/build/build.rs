use std::env;
use std::fs;
use std::path::Path;

use regex::Regex;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config: cbindgen::Config = Default::default();
    config.language = cbindgen::Language::C;

    let header_filepath = Path::new(&crate_dir).join("include/slintwrapper.h");
    match fs::remove_file(&header_filepath) {
        Ok(_) => println!("File {} removed successfully!",&header_filepath.display()),
        Err(e) => println!("Error removing file: {}", e),
    }
    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file("include/slintwrapper.h");
    assert!(header_filepath.exists());

    let header_content = fs::read_to_string(&header_filepath).unwrap();

    // adds a header
    let mut new_header_content =
        "// This file was generated by `build/build.rs` script\n\n".to_owned();
    new_header_content.push_str(&header_content);

    fn word_replace(str: &str) -> String {
        let re = Regex::new(r"App").unwrap();
        re.replace_all(&str, "int32_t").to_string()
    }
    new_header_content = word_replace(&new_header_content);

    fs::write(header_filepath, new_header_content).unwrap();
}
