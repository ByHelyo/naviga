use crate::app::Action;
use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // exit application
        KeyCode::Char('q') => {
            app.running = false;
        }
        // Move cursor left
        KeyCode::Char('h') | KeyCode::Left => {
            app.action = Some(Action::Left);
        }
        // Move cursor right
        KeyCode::Char('l') | KeyCode::Right => {
            app.action = Some(Action::Right);
        }
        // Move cursor up
        KeyCode::Char('k') | KeyCode::Up => {
            app.action = Some(Action::Up);
        }
        // Move cursor down
        KeyCode::Char('j') | KeyCode::Down => {
            app.action = Some(Action::Down);
        }
        _ => {}
    }
    Ok(())
}
