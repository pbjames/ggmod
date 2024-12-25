use crate::files::*;

pub mod cli;
pub mod files;
pub mod gamebanana {
    mod util;
    use util::*;
    pub mod builder;
    pub mod modpage;
    pub mod search;
}
pub mod tui {
    mod app;
    mod runner;
    mod ui;
    pub use runner::run_tui;
}
pub mod modz;
