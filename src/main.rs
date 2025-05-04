mod app;
mod cfg;
mod cli;
mod shared;

use crate::app::infrastructure::command_handler::CommandHandler;
use crate::shared::infrastructure::filesystem::Filesystem;
use cfg::infrastructure::config_repository_impl::ConfigRepository;
use clap::Parser;
use cli::infrastructure::parser::{Cli, Commands};

// cargo run -- generate --path Injection.kt  --name example --pkg com.example
fn main() {
    let cli = Cli::parse();
    let fs = Filesystem;
    let config_repository = ConfigRepository::new(fs);
    let config = config_repository.load();

    let handler = CommandHandler::new(config.clone(), fs);

    match cli.command {
        Commands::Generate { path, name, pkg } => {
            if let Err(e) = handler.generate(path, name, pkg) {
                eprintln!("Error: {}", e);
            }
        }
    }
}
