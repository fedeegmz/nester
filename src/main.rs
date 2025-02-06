mod cli;
mod file_system;
use file_system as fs;
mod project;
mod templates;
mod utils;

fn main() {
    let args = cli::Args::parse();
    let generate = args.generate;
    let name = args.name;

    let project = project::Project::init(&args.path).set_pkg_name();

    match generate {
        cli::Generate::Module => {
            if let Err(e) = fs::create_dir(&project.build_path(&name)) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
            fs::create_kotlin_file(&project, &name, "Injection", templates::INJECTION);
            fs::create_kotlin_file(&project, &name, "Service", templates::SERVICE);
            fs::create_kotlin_file(&project, &name, "Routing", templates::ROUTING);
        }
    }
}
