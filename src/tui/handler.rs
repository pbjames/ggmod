use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::Backend,
    Terminal,
};

use crate::modz::LocalCollection;

use super::{
    app::{App, View, Window},
    state::Itemized,
    ui::show_ui,
};

use anyhow::Result;

pub fn run_tui(collection: &mut LocalCollection) {
    let mut terminal = ratatui::init();
    let app_result = run(&mut terminal, collection);
    ratatui::restore();
    app_result.unwrap();
}

fn run<B: Backend>(terminal: &mut Terminal<B>, collection: &mut LocalCollection) -> Result<()> {
    let mut app = App::new(collection);
    loop {
        terminal.draw(|f| show_ui(f, &mut app))?;
        let res = handle_event(&mut app)?;
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
        if !app.popup_items.is_empty() {
            match key.code {
                KeyCode::Char('j') => app.popup_items.next(),
                KeyCode::Char('k') => app.popup_items.previous(),
                KeyCode::Char('q') => app.popup_items.clear(),
                KeyCode::Esc => app.popup_items.clear(),
                KeyCode::Enter => app.select(),
                _ => (),
            }
            return Ok(false);
        }
        match &app.window.item {
            Window::Search => match key.code {
                KeyCode::Left => app.sort.cycle_back(),
                KeyCode::Right => app.sort.cycle(),
                KeyCode::Enter => app.search()?,
                KeyCode::Backspace => app.backspace(),
                KeyCode::Char(s) => app.type_search(s),
                _ => (),
            },
            other_window => {
                match other_window {
                    Window::Main => match key.code {
                        KeyCode::Char('H') => app.toggle_view(),
                        KeyCode::Char('L') => app.toggle_view(),
                        KeyCode::Char('h') | KeyCode::Left => match app.view {
                            View::Manage(_) => app.toggle_sides(),
                            View::Browse => app.gallery_prev(),
                        },
                        KeyCode::Char('l') | KeyCode::Right => match app.view {
                            View::Manage(_) => app.toggle_sides(),
                            View::Browse => app.gallery_next(),
                        },
                        KeyCode::Char('j') | KeyCode::Down => app.next(),
                        KeyCode::Char('k') | KeyCode::Up => app.previous(),
                        KeyCode::Enter => app.select(),
                        _ => (),
                    },
                    Window::Section => match key.code {
                        KeyCode::Char('j') => app.section.cycle(),
                        KeyCode::Char('k') => app.section.cycle_back(),
                        _ => (),
                    },
                    Window::Category => match key.code {
                        KeyCode::Char('j') => app.categories.next(),
                        KeyCode::Char('k') => app.categories.previous(),
                        _ => (),
                    },
                    _ => (),
                };
                match key.code {
                    KeyCode::Char('1') => app.window.cycle_to(Window::Search),
                    KeyCode::Char('2') => app.window.cycle_to(Window::Main),
                    KeyCode::Char('3') => app.window.cycle_to(Window::Category),
                    KeyCode::Char('4') => app.window.cycle_to(Window::Section),
                    _ => (),
                }
            }
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
