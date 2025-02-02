use clap::Parser;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::str::FromStr;

const MODULE_TEMPLATE: &str = "
package <!pkg_name!>.<!module_name!>

import io.ktor.server.application.*

fun Application.<!module_name!>Module() {
    configure<!module_name_cap!>Routing()
}";

const CONTROLLER_TEMPLATE: &str = "
package <!pkg_name!>.<!module_name!>

import org.koin.core.component.KoinComponent

class <!module_name_cap!>Controller : KoinComponent {}";

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
    fn build_path(&self, module: &str) -> String {
        format!(
            "{}/{}/{}",
            self.root.display().to_string(),
            self.pkg.replace(".", "/"),
            module,
        )
    }
}

#[derive(Clone, clap::ValueEnum, Debug)]
pub enum Generate {
    Module,
}

#[derive(clap::Parser)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value = env!("PWD"))]
    pub path: String,

    #[arg(short = 'g', long = "gen")]
    pub generate: Generate,

    #[arg(short, long)]
    pub name: String,
}

fn main() {
    let args = Args::parse();
    let path = parse_path(args.path.clone());
    let root_path = Path::new(&path);
    let generate = args.generate;
    let name = args.name;

    let pkg_name = match find_pkg_name(root_path) {
        Some(pkg) => pkg,
        None => {
            eprintln!("Package name not found");
            std::process::exit(1);
        }
    };
    let project = Project {
        root: root_path,
        pkg: pkg_name,
    };

    match generate {
        Generate::Module => {
            create_dir(project.build_path(&name));
            create_kotlin_file(&project, &name, "Module", MODULE_TEMPLATE);
            create_kotlin_file(&project, &name, "Controller", CONTROLLER_TEMPLATE);
            create_kotlin_file(&project, &name, "Routing", ROUTING_TEMPLATE);
        }
    }
}

fn create_dir(path: String) {
    let path_obj = Path::new(&path);
    if !path_obj.exists() {
        if let Err(e) = fs::create_dir_all(path_obj) {
            eprintln!("Error creating directories: {}", e);
            std::process::exit(1);
        }
    } else {
        eprintln!("Directory {} already exists", path);
        std::process::exit(1);
    }
}

fn create_kotlin_file(project: &Project, module: &str, file_name: &str, template: &str) {
    let file_path = project.build_path(&format!("/{}/{}.kt", module, file_name));
    let content = String::from_str(template);
    let mut content = content.unwrap();
    content = content
        .replace("<!pkg_name!>", &project.pkg)
        .replace("<!module_name!>", module)
        .replace("<!module_name_cap!>", &capitalize_first_letter(module));

    match fs::File::create(&file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content.as_bytes()) {
                eprintln!("Error writing to file: {}", e);
            } else {
                println!("File created: {}", file_path);
            }
        }
        Err(e) => {
            eprintln!("Error creating file {}: {}", file_path, e);
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
                } else if let Some(file_name) = path.file_name() {
                    if file_name == "Application.kt" {
                        return extract_package_line(&path);
                    }
                }
            }
        }
    }
    eprintln!("Application.kt file with package line not found.");
    std::process::exit(1);
}

fn extract_package_line(file_path: &Path) -> Option<String> {
    let file = fs::File::open(file_path).ok()?;
    let reader = io::BufReader::new(file);
    for line in reader.lines().flatten() {
        if line.starts_with("package ") {
            return Some(line.replace("package ", ""));
        }
    }
    None
}

fn parse_path(path: String) -> String {
    const KTOR_SRC: &str = "/src/main/kotlin";
    let path_values = path.split("/");
    let mut parsed_path = String::new();
    for part in path_values {
        if !part.is_empty() {
            parsed_path.push('/');
            parsed_path.push_str(part);
        }
    }
    parsed_path += KTOR_SRC;
    return parsed_path;
}

fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => s.to_string(),
    }
}
