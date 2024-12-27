use std::io;

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::Backend,
    Terminal,
};

use crate::modz::LocalCollection;

use super::{
    app::{App, Window},
    ui::ui,
};

type Result<T> = std::result::Result<T, io::Error>;

pub fn run_tui(collection: &mut LocalCollection) {
    let mut terminal = ratatui::init();
    let mut app = App::new(collection);
    let app_result = run(&mut terminal, &mut app);
    ratatui::restore();
    app_result.unwrap();
}

fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        let res = handle_event(app)?;
        if res {
            break;
        }
    }
    Ok(())
}

fn handle_event(app: &mut App) -> Result<bool> {
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Release {
            return Ok(false);
        }
        match app.window {
            Window::Main => match key.code {
                KeyCode::Char('h') => app.toggle_view(),
                KeyCode::Char('j') => app.scroll_down(),
                KeyCode::Char('k') => app.scroll_up(),
                KeyCode::Char('l') => app.toggle_view(),
                _ => (),
            },
            Window::Search => match key.code {
                KeyCode::Enter => app.search(),
                KeyCode::Backspace => {
                    app.search.pop();
                }
                KeyCode::Left => app.cycle_sort_back(),
                KeyCode::Right => app.cycle_sort(),
                KeyCode::Char(s) => app.search.push(s),
                _ => (),
            },
            Window::Section => (),
            Window::Category => (),
        }
        match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Tab => app.cycle_window(),
            KeyCode::BackTab => app.cycle_window_back(),
            _ => (),
        }
    }
    Ok(false)
}
