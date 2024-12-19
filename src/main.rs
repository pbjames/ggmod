use clap::{Parser, Subcommand};
use ggmod::files::{check_registry, load_mods};
use ggmod::gamebanana::{GBMod, Mod};
use std::io;
use std::path::PathBuf;
use std::process::exit;

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
    match &cli.command {
        Some(Commands::Download { mod_id, install }) => {
            let gbmod = GBMod::build(*mod_id).expect("Couldn't get mod");
            let opts = gbmod.files();
            for f in opts {
                println!("{:?}", f._sFile);
            }
            print!("Choose index:");
            let mut input = String::new();
            //io::stdin()
            //    .read_line(&mut input)
            //    .expect("Unable to read line");
            input = String::from("1");
            let chosen_mod = Mod::build(*mod_id, gbmod, input.trim().parse().unwrap())
                .expect("Couldn't download file");
            exit(0);
            if *install {
                chosen_mod.stage().expect("Mod couldn't be staged");
            }
        }
        Some(Commands::Install { mod_id }) => {}
        Some(Commands::Uninstall { mod_id }) => {}
        Some(Commands::List {}) => {
            let mods: Vec<Mod> = load_mods(reg_path).expect("Mods couldn't be loaded");
            for m in mods {
                println!("{}: {}", m.id, m.name)
            }
        }
        None => {}
    }
}
