use clap::Parser;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

const MODULE_TEMPLATE: &str = "
package <!pkg_name!>.<!module_name!>

import io.ktor.server.application.*

fun Application.<!module_name!>Module() {
    configure<!module_name_cap!>Routing()
}";

const SERVICE_TEMPLATE: &str = "
package <!pkg_name!>.<!module_name!>

import org.koin.core.component.KoinComponent

class <!module_name_cap!>Service : KoinComponent {}";

const ROUTING_TEMPLATE: &str = "
package <!pkg_name!>.<!module_name!>

import io.ktor.server.application.*
import io.ktor.server.routing.*

fun Application.configure<!module_name_cap!>Routing() {
    routing {
        route(\"/<!module_name!>\") {

        }
    }
}
";

struct Project<'a> {
    root: &'a Path,
    pkg: String,
}

impl<'a> Project<'a> {
    fn build_path(&self, module: &str) -> PathBuf {
        let full_path = self.root.join(self.pkg.replace('.', "/"));
        if full_path.is_dir() {
            full_path.join(module)
        } else {
            self.root.join(module)
        }
    }
}

#[derive(Clone, clap::ValueEnum, Debug)]
pub enum Generate {
    Module,
}

#[derive(clap::Parser)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = std::env::current_dir()
        .expect("Failed to get current directory")
        .to_str()
        .expect("Invalid UTF-8 in path")
        .to_string())]
    pub path: String,

    #[arg(short, long)]
    pub generate: Generate,

    #[arg(short, long)]
    pub name: String,
}

fn main() {
    let args = Args::parse();
    let path = parse_path(args.path);
    let generate = args.generate;
    let name = args.name;

    let pkg_name = find_pkg_name(&path).unwrap_or_else(|| {
        eprintln!("Error: Could not determine package name. Make sure `Application.kt` exists.");
        std::process::exit(1);
    });

    let project = Project {
        root: &path,
        pkg: pkg_name,
    };

    match generate {
        Generate::Module => {
            if let Err(e) = create_dir(&project.build_path(&name)) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
            create_kotlin_file(&project, &name, "Module", MODULE_TEMPLATE);
            create_kotlin_file(&project, &name, "Service", SERVICE_TEMPLATE);
            create_kotlin_file(&project, &name, "Routing", ROUTING_TEMPLATE);
        }
    }
}

fn create_dir(path: &Path) -> Result<(), String> {
    if !path.exists() {
        fs::create_dir_all(path).map_err(|e| format!("Error creating directories: {}", e))
    } else {
        Err(format!("Directory {} already exists", path.display()))
    }
}

fn create_kotlin_file(project: &Project, module: &str, file_name: &str, template: &str) {
    let file_path = project.build_path(&format!("{}/{}.kt", module, file_name));
    let content = template
        .replace("<!pkg_name!>", &project.pkg)
        .replace("<!module_name!>", module)
        .replace("<!module_name_cap!>", &capitalize_first_letter(module));

    match fs::File::create(&file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content.as_bytes()) {
                eprintln!("Error writing to file: {}", e);
            } else {
                println!("File created: {}", file_path.display());
            }
        }
        Err(e) => {
            eprintln!("Error creating file {}: {}", file_path.display(), e);
            std::process::exit(1);
        }
    }
}

fn find_pkg_name(root_path: &Path) -> Option<String> {
    let mut stack = vec![root_path.to_path_buf()];
    while let Some(current_path) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&current_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.file_name()? == "Application.kt" {
                    return extract_package_line(&path);
                }
            }
        }
    }
    None
}

fn extract_package_line(file_path: &Path) -> Option<String> {
    let file = fs::File::open(file_path).ok()?;
    let reader = io::BufReader::new(file);
    reader
        .lines()
        .flatten()
        .find_map(|line| line.strip_prefix("package ").map(|pkg| pkg.to_string()))
}

fn parse_path(path: String) -> PathBuf {
    PathBuf::from(path).join("src/main/kotlin")
}

fn capitalize_first_letter(s: &str) -> String {
    s.chars().next().map_or_else(String::new, |first| {
        format!("{}{}", first.to_uppercase(), &s[1..])
    })
}
