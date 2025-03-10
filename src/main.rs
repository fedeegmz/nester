mod cli;
mod config;
mod file_system;
mod init;
mod project;
mod templates;
mod utils;

use config::load_config;
use init::init;
use project::Project;
use templates::get_content;

fn main() {
    let config = load_config();

    if let Err(e) = init(&config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    let args = cli::Args::parse();
    let generate = args.generate;
    let name = args.name;

    let project = Project::init(&args.path).set_pkg_name();

    match generate {
        cli::Generate::Module => {
            if let Err(e) = file_system::create_dir(&project.build_path(&name)) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
            for file in &config.ktor.module_files {
                match get_content(&project, file.template.as_str(), &name) {
                    Ok(content) => file_system::create_kotlin_file(
                        &project,
                        &name,
                        file.name.as_str(),
                        content,
                    ),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}
