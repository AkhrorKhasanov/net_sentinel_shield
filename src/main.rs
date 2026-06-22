mod cli;
mod hosts;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    #[cfg(target_os = "windows")]
    if !is_admin() {
        eprintln!("Error: Please run as Administrator!");
        std::process::exit(1);
    }

    match args.command {
        Commands::Block { file } => hosts::block_domains(&file),
        Commands::Restart => hosts::restore_hosts(),
    }
}

#[cfg(target_os = "windows")]
fn is_admin() -> bool {
    is_elevated::is_elevated()
}
