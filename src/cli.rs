use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum, Debug)]
pub enum Generate {
    Module,
}

#[derive(Parser)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = std::env::current_dir()
        .expect("❌ Failed to get current directory")
        .to_str()
        .expect("❌ Invalid UTF-8 in path")
        .to_string())]
    pub path: String,

    #[arg(short, long)]
    pub generate: Generate,

    #[arg(short, long)]
    pub name: String,
}

impl Args {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
