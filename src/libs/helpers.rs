use path_absolutize::Absolutize;
use std::path::Path;

pub fn to_abs_path(path: &str) -> String {
    Path::new(&path).absolutize().unwrap().to_str().unwrap().to_string()
}
