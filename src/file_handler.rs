use crate::templates::*;
use std::fs::{create_dir, remove_dir_all, File};
use std::io::Read;
use std::io::Write;
use std::path::Path;

const OUTPUT_DIR: &str = "output";

pub(crate) fn initialize_output_dir(bin_nm: &str) -> File {
    let mut path = Path::new(OUTPUT_DIR).to_path_buf();
    if path.exists() {
        remove_dir_all(path.clone()).unwrap();
    }
    create_dir(path.clone()).unwrap();
    path.push("Cargo.toml");
    let mut file = File::create(path.clone()).unwrap();
    let toml_content = get_cargo_toml_content(bin_nm);
    write_to_file(&mut file, &toml_content);
    path.pop();
    path.push("src");
    create_dir(path.clone()).unwrap();
    path.push("main.rs");
    File::create(path).unwrap()
}

pub(crate) fn write_to_file(file: &mut File, data: &str) {
    file.write_all(data.as_bytes()).unwrap();
}

pub(crate) fn finish_output(file: &mut File) {
    file.flush().unwrap();
}

pub fn read_source_file(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
