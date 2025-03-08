mod cli;
mod file_system;
use file_system as fs;
use templates::get_content;
mod constants;
mod init;
mod project;
mod templates;
mod utils;

fn main() {
    if let Err(e) = init::init() {
        eprintln!("{}", e);
        std::process::exit(1);
    }

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
            fs::create_kotlin_file(
                &project,
                &name,
                "Injection",
                match get_content(&project, constants::INJECTION_TEMPLATE_NAME, &name) {
                    Ok(content) => content,
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    }
                },
            );
            fs::create_kotlin_file(
                &project,
                &name,
                "Service",
                match get_content(&project, constants::SERVICE_TEMPLATE_NAME, &name) {
                    Ok(content) => content,
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    }
                },
            );
            fs::create_kotlin_file(
                &project,
                &name,
                "Routing",
                match get_content(&project, constants::ROUTING_TEMPLATE_NAME, &name) {
                    Ok(content) => content,
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    }
                },
            );
        }
    }
}
