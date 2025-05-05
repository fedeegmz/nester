mod app;
mod cfg;
mod cli;
mod core;
mod shared;

use crate::core::port::logger_port::LoggerPort;
use crate::shared::infrastructure::logger::Logger;
use app::infrastructure::command_handler::CommandHandler;
use cfg::infrastructure::config_repository::ConfigRepository;
use cli::infrastructure::parser::{parse, Commands};
use core::port::filesystem_port::FilesystemPort;
use core::port::repository_port::RepositoryPort;
use core::port::templates_port::TemplatesPort;
use shared::infrastructure::filesystem::Filesystem;
use shared::infrastructure::repository::Repository;
use shared::infrastructure::templates::Templates;

fn main() {
    let logger: Box<dyn LoggerPort> = Box::new(Logger::new());
    let cli = parse();
    let fs: Box<dyn FilesystemPort> = Box::new(Filesystem::new(logger.as_ref()));
    let config_repository = ConfigRepository::new(fs.as_ref(), logger.as_ref());
    let config = config_repository.load();
    let repository: Box<dyn RepositoryPort> = Box::new(Repository::new(logger.as_ref()));
    let templates: Box<dyn TemplatesPort> = Box::new(Templates::new(
        config.clone(),
        fs.as_ref(),
        repository.as_ref(),
        logger.as_ref(),
    ));

    let handler = CommandHandler::new(
        config.clone(),
        fs.as_ref(),
        templates.as_ref(),
        repository.as_ref(),
        logger.as_ref(),
    );

    match cli.command {
        Commands::Generate { path, name, pkg } => handler.generate(path, name, pkg),

        Commands::Pull => handler.pull_templates(),
    }
}
