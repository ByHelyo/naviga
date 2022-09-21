mod action;
mod directory;
mod entry;
mod utils;

use directory::Directory;
use std::{env, error, path::PathBuf};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    terminal::Frame,
};

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Action {
    Left,
    Right,
    Up,
    Down,
    Enter,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub action: Option<Action>,
    pub current_directory: Option<Directory>,
    pub previous_directory: Option<Directory>,
    pub next_directory: Option<Directory>,
}

impl Default for App {
    fn default() -> Self {
        let current_path: PathBuf = env::current_dir().unwrap();

        let mut current_directory: Directory = Directory::new(&current_path);
        current_directory.set_state(Some(0));

        let mut app: App = App {
            running: true,
            action: None,
            current_directory: Some(current_directory),
            previous_directory: None,
            next_directory: None,
        };

        app.build_previous_dir();
        app.build_next_dir();

        app
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        // This is where you add new widgets.
        // See the following resources:
        // - https://docs.rs/tui/0.16.0/tui/widgets/index.html
        // - https://github.com/fdehau/tui-rs/tree/v0.16.0/examples

        let main_chunks: Vec<Rect> = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
            .split(frame.size());

        let directories_chunks: Vec<Rect> = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Length(1),
                    Constraint::Percentage(40),
                    Constraint::Length(1),
                    Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(main_chunks[1]);

        self.handle_action();

        self.current_directory
            .as_ref()
            .unwrap()
            .render_current_directory_path(frame, &main_chunks[0]);

        if let Some(directory) = &mut self.previous_directory {
            directory.render_directory(frame, &directories_chunks[0]);
        }

        self.current_directory
            .as_mut()
            .unwrap()
            .render_directory(frame, &directories_chunks[2]);

        if let Some(directory) = &mut self.next_directory {
            directory.render_directory(frame, &directories_chunks[4]);
        }

        self.action = None;
    }
}
