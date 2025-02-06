use std::sync::Arc;

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::Backend,
    Terminal,
};
use tokio::sync::{broadcast::Receiver, Mutex};

use crate::modz::LocalCollection;

use super::{
    app::{App, View, Window},
    state::Itemized,
    termination::Termination,
    ui::show_ui,
};

type Am<T> = Arc<Mutex<T>>;

pub async fn run_tui(collection: LocalCollection) {
    let terminal = Arc::new(Mutex::new(ratatui::init()));
    let app = Arc::new(Mutex::new(App::new(collection).await));
    let app_copy = app.clone();
    let (termination, rx_terminate) = Termination::new();
    let rx_terminate_copy = rx_terminate.resubscribe();
    tokio::spawn(async move { draw_loop(terminal, app_copy, rx_terminate).await });
    event_loop(app, termination, rx_terminate_copy).await;
}

async fn event_loop(app: Arc<Mutex<App>>, term: Termination, mut rx_term: Receiver<usize>) {
    loop {
        if rx_term.try_recv().unwrap_or(0) == 1 {
            ratatui::restore();
            break;
        }
        let mut appref = app.lock().await;
        handle_event(&mut appref, &term).await
    }
}

async fn draw_loop<B: Backend>(
    terminal: Am<Terminal<B>>,
    app: Am<App>,
    mut rx_term: Receiver<usize>,
) {
    loop {
        if rx_term.try_recv().unwrap_or(0) == 1 {
            ratatui::restore();
            break;
        }
        let mut appref = app.lock().await;
        terminal
            .lock()
            .await
            .draw(|f| {
                show_ui(f, &mut appref);
            })
            .unwrap();
    }
}

async fn handle_event(app: &mut App, term: &Termination) {
    if let Event::Key(key) = event::read().unwrap() {
        if key.kind == event::KeyEventKind::Release {
            return;
        }
        if !app.popup_items.is_empty() {
            match key.code {
                KeyCode::Char('j') => app.popup_items.next(),
                KeyCode::Char('k') => app.popup_items.previous(),
                KeyCode::Char('q') => app.popup_items.clear(),
                KeyCode::Esc => app.popup_items.clear(),
                KeyCode::Enter => app.select().await,
                _ => (),
            }
            return;
        }
        match &app.window.item {
            Window::Search => match key.code {
                KeyCode::Left => app.sort.cycle_back(),
                KeyCode::Right => app.sort.cycle(),
                KeyCode::Enter => app.search().await.unwrap(),
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
                        KeyCode::Enter => app.select().await,
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
            KeyCode::Esc => term.exit(),
            KeyCode::Tab => app.window.cycle(),
            KeyCode::BackTab => app.window.cycle_back(),
            _ => (),
        }
    }
}
