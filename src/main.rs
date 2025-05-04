mod app;
mod cfg;
mod cli;
mod core;
mod shared;

use app::infrastructure::command_handler::CommandHandler;
use cfg::infrastructure::config_repository::ConfigRepository;
use cli::infrastructure::parser::{parse, Commands};
use core::port::filesystem_port::FilesystemPort;
use core::port::templates_port::TemplatesPort;
use shared::infrastructure::filesystem::Filesystem;
use shared::infrastructure::templates::Templates;

// cargo run -- generate --path Injection.kt  --name example --pkg com.example
fn main() {
    let cli = parse();
    let fs: Box<dyn FilesystemPort> = Box::new(Filesystem);
    let config_repository = ConfigRepository::new(fs.as_ref());
    let config = config_repository.load();
    let templates: Box<dyn TemplatesPort> = Box::new(Templates::new(config.clone(), fs.as_ref()));

    let handler = CommandHandler::new(config.clone(), fs.as_ref(), templates.as_ref());

    match cli.command {
        Commands::Generate { path, name, pkg } => {
            if let Err(e) = handler.generate(path, name, pkg) {
                eprintln!("Error: {}", e);
            }
        }
    }
}
