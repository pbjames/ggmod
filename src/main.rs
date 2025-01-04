use std::cell::RefCell;

use clap::{Parser, Subcommand};
use ggmod::cli::*;
use ggmod::modz::LocalCollection;
use ggmod::tui::run_tui;
use log::LevelFilter;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = false)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

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
        /// Mod ID
        mod_id: usize,
    },
    /// Puts mod inside GGST mod folder
    Install { mod_id: usize },

    /// Can be re-installed again
    Uninstall { mod_id: usize },

    /// List mods and respective IDs
    List {},

    /// Search online page
    Search {
        /// Number of results per page
        #[arg(short, long)]
        size: Option<usize>,

        /// Sort by featured
        #[arg(short, long)]
        featured: bool,

        /// Sort by popularity
        #[arg(short, long)]
        popular: bool,

        /// Sort by time
        #[arg(short, long)]
        recent: bool,

        /// Page no. starting from 1
        page: usize,

        /// Search by name
        name: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let collection = LocalCollection::new();
    simple_logging::log_to_file(
        "log.txt",
        match cli.verbose {
            0 => LevelFilter::Off,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        },
    )
    .expect("Couldn't setup logging");
    match &cli.command {
        Some(Commands::Download { mod_id, install }) => download(collection, *mod_id, *install),
        Some(Commands::Install { mod_id }) => install(collection, *mod_id),
        Some(Commands::Uninstall { mod_id }) => uninstall(collection, *mod_id),
        Some(Commands::List {}) => list_all(collection),
        Some(Commands::Search {
            page,
            size: page_size,
            name,
            featured,
            popular,
            recent,
        }) => search(
            *page,
            *page_size,
            name.clone(),
            *featured,
            *popular,
            *recent,
        ),
        None => run_tui(RefCell::new(collection)),
    }
}
