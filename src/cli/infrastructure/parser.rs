use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "nester-cli")]
#[command(about = "CLI tool to generate project templates", long_about = None)]
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
}
