use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "NetSentinel Shield")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Block { file: String },
    Restart,
}