use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "nester")]
#[command(about = "CLI tool to generate project templates", long_about = None)]
#[command(version = "1.0.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Generate {
        #[arg(short, long)]
        path: String,

        #[arg(long)]
        name: Option<String>,

        #[arg(long)]
        pkg: Option<String>,
    },
    Pull,
}

pub fn parse() -> Cli {
    Cli::parse()
}
