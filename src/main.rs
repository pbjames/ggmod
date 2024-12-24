use clap::{Parser, Subcommand};
use ggmod::gamebanana::builder::SearchBuilder;
use ggmod::gamebanana::modpage::GBModPage;
use ggmod::modz::LocalCollection;
use std::io::{self, BufRead};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
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
        #[arg(short, long)]
        page_size: usize,
        page: usize,
    },
}

fn main() {
    let cli = Cli::parse();
    let collection = LocalCollection::new();
    match cli.verbose {
        0 => (),
        1 => colog::init(),
        2 => colog::default_builder()
            .filter_level(log::LevelFilter::Debug)
            .init(),
        _ => colog::default_builder()
            .filter_level(log::LevelFilter::Trace)
            .init(),
    }
    match &cli.command {
        Some(Commands::Download { mod_id, install }) => download(collection, *mod_id, *install),
        Some(Commands::Install { mod_id }) => install(collection, *mod_id),
        Some(Commands::Uninstall { mod_id }) => uninstall(collection, *mod_id),
        Some(Commands::List {}) => list_all(collection),
        Some(Commands::Search { page_size, page }) => search(*page_size, *page),
        None => {}
    }
}
fn search(size: usize, page: usize) {
    let entries = SearchBuilder::new()
        .per_page(size)
        .build()
        .read_page(page)
        .expect("Couldn't get search results");
    for entry in entries {
        let mut name = entry.name.clone();
        name.truncate(20);
        let mut desc = entry.description.clone();
        desc.truncate(20);
        let views = entry.view_count;
        println!("{name:<20} - {desc:<20} :: {views} views");
    }
}

fn list_all(col: LocalCollection) {
    for mod_ in col.mods() {
        println!(
            "[{}] [{}] {}: {}",
            if mod_.staged { "+" } else { " " },
            mod_.character,
            mod_.id,
            mod_.name
        )
    }
}

fn download(mut col: LocalCollection, mod_id: usize, do_install: bool) {
    let gbmod = GBModPage::build(mod_id).expect("Couldn't get online mod page");
    let opts = &gbmod.files;
    for (i, f) in opts.iter().enumerate() {
        println!("[{}] {:?}", (i + 1), f.file);
    }
    println!("Choose index:");
    let input = choose_num() - 1;
    col.register_online_mod(gbmod, input)
        .expect("Couldn't download mod");
    if do_install {
        install(col, mod_id)
    }
}

fn install(mut col: LocalCollection, mod_id: usize) {
    col.apply_on_mod(
        mod_id,
        Box::new(|mod_| mod_.stage().expect("Couldn't add mod to GGST")),
    )
    .expect("add ");
}

fn uninstall(mut col: LocalCollection, mod_id: usize) {
    col.apply_on_mod(
        mod_id,
        Box::new(|mod_| mod_.unstage().expect("Couldn't remove mod from GGST")),
    )
    .expect("couldnt rempve stuf");
}

fn choose_num() -> usize {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let input = iterator.next().unwrap().unwrap();
    input.trim().parse::<usize>().unwrap()
}
