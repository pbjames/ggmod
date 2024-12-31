use crate::files::*;

pub mod cli;
pub mod files;
pub mod gamebanana {
    mod util;
    use util::*;
    pub mod builder;
    pub mod models {
        pub mod category;
        pub mod file;
        pub mod game;
        pub mod modpage;
        pub mod preview;
        pub mod search_result;
    }
    pub mod search;
}
pub mod tui {
    mod app;
    mod handler;
    mod search;
    mod ui;
    pub use handler::run_tui;
}
pub mod modz;
