use clap::Parser;
use std::fs::{self};

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
    let generate = args.generate;
    let name = args.name;

    let domain_dirs = read_entries(path.clone());
    let pkg_dirs = read_entries(domain_dirs[0].clone());
    let pkg_dir = pkg_dirs[0].clone();
    match generate {
        Generate::Module => {
            let module_path = format!("{}/{}", pkg_dir, name);
            create_dir(module_path)
        }
    }
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

fn read_entries(path: String) -> Vec<String> {
    let mut dirs = vec![];
    let entries = fs::read_dir(path.clone());
    match entries {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let dir = entry.path().display().to_string();
                        dirs.push(dir);
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        }
        Err(_) => eprintln!("Error: Invalid directory {}", path),
    }
    return dirs;
}

fn create_dir(path: String) {
    match fs::create_dir_all(&path) {
        Ok(_) => println!("Created directory: {}", path),
        Err(e) => eprintln!("Error creating directory {}: {}", path, e),
    }
}
