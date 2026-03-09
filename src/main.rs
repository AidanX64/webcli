mod app;
mod catalog;
mod ui;
mod wishlist;

use std::io;

use app::{App, Screen};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::DefaultTerminal;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let terminal = ratatui::init();
    let result = run_app(terminal);
    ratatui::restore();
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    result
}

fn run_app(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut app = App::default();

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if app.wishlist_form.is_some() {
                match key.code {
                    KeyCode::Esc => app.cancel_wishlist_form(),
                    KeyCode::Enter => app.submit_wishlist_field(),
                    KeyCode::Backspace => app.pop_input_char(),
                    KeyCode::Char(ch)
                        if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT =>
                    {
                        app.input_char(ch)
                    }
                    _ => {}
                }
                continue;
            }

            match key.code {
                KeyCode::Char('q') => {
                    if app.persist_wishlist().is_ok() {
                        return Ok(());
                    }
                }
                KeyCode::Char('w') => app.open_wishlist(),
                KeyCode::Char('a') if app.screen == Screen::ProductDetail => {
                    app.add_to_wishlist_from_catalog()
                }
                KeyCode::Char('n') if app.screen == Screen::Wishlist => app.start_wishlist_form(),
                KeyCode::Char('d') | KeyCode::Delete if app.screen == Screen::Wishlist => {
                    app.delete_selected_wishlist_item()
                }
                KeyCode::Down | KeyCode::Char('j') => app.next(),
                KeyCode::Up | KeyCode::Char('k') => app.previous(),
                KeyCode::Enter => app.select(),
                KeyCode::Esc | KeyCode::Backspace => app.back(),
                _ => app.clear_status(),
            }
        }
    }
}
