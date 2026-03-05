use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

pub fn find_package_declaration() -> Option<String> {
    let extensions = ["kt", "java", "dart"];

    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if extensions.contains(&ext) {
                if let Ok(file) = File::open(path) {
                    let reader = BufReader::new(file);
                    for line in reader.lines().flatten() {
                        if let Some(rest) = line.strip_prefix("package ") {
                            return Some(rest.trim().to_string());
                        }
                    }
                }
            }
        }
    }

    None
}

pub fn find_pkg_name(root_path: &Path) -> Option<String> {
    println!("root_path: {:?}", root_path);
    let mut stack = vec![root_path.to_path_buf()];
    while let Some(current_path) = stack.pop() {
        if let Ok(entries) = read_dir(&current_path) {
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
    let file = File::open(file_path).ok()?;
    let reader = BufReader::new(file);
    reader
        .lines()
        .flatten()
        .find_map(|line| line.strip_prefix(prefix).map(|pkg| pkg.to_string()))
}
