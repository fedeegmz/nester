use crate::utils::find_pkg_name;
use std::path::{Path, PathBuf};

pub struct Project<'a> {
    pub root_path: &'a Path,
    pub package_name: Option<String>,
    src_path: PathBuf,
}

impl<'a> Project<'a> {
    pub fn init(root_path: &'a str) -> Self {
        let root = Path::new(root_path);
        let src_path = root.join("src/main/kotlin");
        Project {
            root_path: root,
            src_path,
            package_name: None,
        }
    }

    pub fn set_pkg_name(mut self) -> Self {
        let pkg_name = find_pkg_name(&self.root_path).unwrap_or_else(|| {
            eprintln!(
                "❌ Error: Could not determine package name. Make sure `Application.kt` exists."
            );
            std::process::exit(1);
        });
        self.package_name = Some(pkg_name.clone());
        self
    }

    pub fn build_path(&self, path: &str) -> PathBuf {
        let full_path = self
            .src_path
            .join(self.package_name.as_ref().unwrap().replace('.', "/"));
        if full_path.is_dir() {
            full_path.join(path)
        } else {
            self.src_path.join(path)
        }
    }
}
