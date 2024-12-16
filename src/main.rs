use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Provide more debugging information
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Download mod from GB
    Download {
        /// Also install or no
        #[arg(short, long)]
        install: bool,
    },
    /// dragon install
    Install {},

    /// Uninstall mod - can be installed again later
    Uninstall {},
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Download { install: list }) => {}
        Some(Commands::Install {}) => {}
        Some(Commands::Uninstall {}) => {}
        None => {}
    }
}
