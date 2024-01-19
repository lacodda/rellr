use path_absolutize::Absolutize;
use regex::Regex;
use std::{fs, path::Path};

pub fn to_abs_path(path: &str) -> String {
    Path::new(&path).absolutize().unwrap().to_str().unwrap().to_string()
}

pub fn to_path_str(parts: Vec<&str>) -> String {
    let path_str = parts.join("/").replace("\\", "/");
    let regex = Regex::new(r"^(\.?\/)+").unwrap();
    regex.replace(&path_str, "").to_string()
}

pub fn check_files_existence(files: Vec<&str>) -> Vec<String> {
    let mut non_existent_files = vec![];
    for file in files {
        if !fs::metadata(file).is_ok() {
            non_existent_files.push(file.to_string());
        }
    }

    non_existent_files
}
