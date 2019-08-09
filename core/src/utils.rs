use std::fs::canonicalize;
use std::path::{PathBuf, Path};
use std::env;

pub fn relative_path(relative: &str, target: &[&str]) -> PathBuf {
    let current_file = relative.to_owned().to_string();
    let mut base_path = canonicalize(&current_file)
        .expect("").parent()
        .unwrap().to_str().unwrap().to_string();

    let mut path = Path::new(&base_path).to_path_buf();
    for s in target {
        path.push(*s)
    }
    path
}

pub fn relative_to_current_path(target: &[&str]) -> PathBuf {
    let mut base_path = env::current_dir().unwrap();

    let mut path = base_path.to_path_buf();
    for s in target {
        path.push(*s)
    }
    path
}