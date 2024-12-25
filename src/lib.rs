use crate::files::*;

mod tui;
pub use tui::run_tui;
pub mod cli;
pub mod files;
pub mod gamebanana {
    mod util;
    use util::*;
    pub mod builder;
    pub mod modpage;
    pub mod search;
}
pub mod modz;
