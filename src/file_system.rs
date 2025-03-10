use crate::project::Project;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn create_dir(path: &Path) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        println!("⚠️ Directory {} already exists", path.display());
    }
    if let Err(e) = fs::create_dir_all(path) {
        return Err(Box::new(e));
    }

    Ok(())
}

pub fn create_kotlin_file(project: &Project, module: &str, file_name: &str, content: String) {
    let file_path = project.build_path(&format!("{}/{}.kt", module, file_name));

    match fs::File::create(&file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content.as_bytes()) {
                eprintln!("⚠️ Error writing to file: {}", e);
            } else {
                println!("✅ File created: {}", file_path.display());
            }
        }
        Err(e) => {
            eprintln!("❌ Error creating file {}: {}", file_path.display(), e);
            std::process::exit(1);
        }
    }
}
