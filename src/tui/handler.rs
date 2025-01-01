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

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
        match app.window.item {
            Window::Main => match key.code {
                KeyCode::Char('h') => app.toggle_view(),
                KeyCode::Char('j') => app.next(),
                KeyCode::Char('k') => app.previous(),
                KeyCode::Char('l') => app.toggle_view(),
                _ => (),
            },
            Window::Search => match key.code {
                KeyCode::Left => app.sort.cycle_back(),
                KeyCode::Right => app.sort.cycle(),
                KeyCode::Enter => app.search()?,
                KeyCode::Backspace => app.backspace(),
                KeyCode::Char(s) => app.type_search(s),
                _ => (),
            },
            Window::Section => match key.code {
                KeyCode::Char('j') => app.section.cycle(),
                KeyCode::Char('k') => app.section.cycle_back(),
                _ => (),
            },
            Window::Category => (),
        }
        match key.code {
            KeyCode::Esc => return Ok(true),
            KeyCode::Tab => app.window.cycle(),
            KeyCode::BackTab => app.window.cycle_back(),
            _ => (),
        }
    }
    Ok(false)
}
