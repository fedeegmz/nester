use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub fn find_pkg_name(root_path: &Path) -> Option<String> {
    let mut stack = vec![root_path.to_path_buf()];
    while let Some(current_path) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&current_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.file_name()? == "Application.kt" {
                    return extract_line(&path, "package ");
                }
            }
        }
    }
    None
}

fn extract_line(file_path: &Path, prefix: &str) -> Option<String> {
    let file = fs::File::open(file_path).ok()?;
    let reader = io::BufReader::new(file);
    reader
        .lines()
        .flatten()
        .find_map(|line| line.strip_prefix(prefix).map(|pkg| pkg.to_string()))
}
