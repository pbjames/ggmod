use clap::{Parser, Subcommand};
use ggmod::gamebanana::GBMod;
use ggmod::modz::{check_registry, load_mods, registry_has_id, Mod};
use std::io::{self, BufRead};
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
        /// Mod ID
        mod_id: usize,
    },
    /// dragon install
    Install { mod_id: usize },

    /// Uninstall mod - can be installed again later
    Uninstall { mod_id: usize },

    /// List mods and respective IDs
    List {},
}

fn main() {
    let cli = Cli::parse();
    let reg_path = check_registry().unwrap_or_else(|e| {
        panic!("{}", e);
    });
    colog::init();
    match &cli.command {
        Some(Commands::Download { mod_id, install }) => {
            let gbmod = GBMod::build(*mod_id).expect("Couldn't get mod");
            let opts = &gbmod.files;
            for (i, f) in opts.iter().enumerate() {
                println!("{} {:?}", (i + 1), f.file);
            }
            println!("Choose index:");
            let stdin = io::stdin();
            let mut iterator = stdin.lock().lines();
            let input = iterator.next().unwrap().unwrap();
            let input = input.trim().parse::<usize>().unwrap() - 1;
            let mut chosen_mod = Mod::build(*mod_id, gbmod, input).expect("Couldn't download file");
            if *install {
                chosen_mod.stage().expect("Mod couldn't be staged");
            }
        }
        Some(Commands::Install { mod_id }) => {
            let mut chosen_mod = registry_has_id(*mod_id)
                .expect("Invalid ID or registry (delete it)")
                .expect("Couldn't find mod with that ID");
            chosen_mod.stage().expect("Couldn't add mod to GGST");
        }
        Some(Commands::Uninstall { mod_id }) => {
            let mut chosen_mod = registry_has_id(*mod_id)
                .expect("Invalid ID or registry (delete it)")
                .expect("Couldn't find mod with that ID");
            chosen_mod.unstage().expect("Couldn't remove mod");
        }
        Some(Commands::List {}) => {
            let mods: Vec<Mod> = load_mods(&reg_path).expect("Mods couldn't be loaded");
            for m in mods {
                println!("{}: {}", m.id, m.name)
            }
        }
        None => {}
    }
}
