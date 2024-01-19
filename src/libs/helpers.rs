use path_absolutize::Absolutize;
use regex::Regex;
use std::path::Path;

pub fn to_abs_path(path: &str) -> String {
    Path::new(&path).absolutize().unwrap().to_str().unwrap().to_string()
}

pub fn to_path_str(parts: Vec<&str>) -> String {
    let path_str = parts.join("/").replace("\\", "/");
    let regex = Regex::new(r"^(\.?\/)+").unwrap();
    regex.replace(&path_str, "").to_string()
}
