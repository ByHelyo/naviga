pub mod directory;
mod utils;

use directory::Directory;
use std::error;
use std::path::PathBuf;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, List, ListItem};

use std::env;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Action {
    Left,
    Right,
    Up,
    Down,
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

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> AppResult<Self> {
        let current_path: PathBuf = env::current_dir()?;

        let mut current_directory: Directory = Directory::new(&current_path)?;
        current_directory.state.select(Some(0));

        let mut app: App = App {
            running: true,
            action: None,
            current_directory: Some(current_directory),
            previous_directory: None,
            next_directory: None,
        };

        app.build_previous_dir()?;
        app.build_next_dir()?;

        Ok(app)
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        // This is where you add new widgets.
        // See the following resources:
        // - https://docs.rs/tui/0.16.0/tui/widgets/index.html
        // - https://github.com/fdehau/tui-rs/tree/v0.16.0/examples

        let size = frame.size();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                    Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(size);

        match self.action {
            Some(Action::Left) => {
                if self.previous_directory.is_some() {
                    self.next_directory = self.current_directory.take();
                    self.next_directory.as_mut().unwrap().state.select(None);

                    self.current_directory = self.previous_directory.take();

                    self.build_previous_dir();
                }
            }
            Some(Action::Right) => {}
            Some(Action::Up) => {
                self.current_directory.as_mut().unwrap().previous();
            }
            Some(Action::Down) => {
                self.current_directory.as_mut().unwrap().next();
            }
            _ => {}
        }

        if let Some(directory) = &mut self.previous_directory {
            App::render_directory(frame, &chunks[0], directory);
        }

        App::render_directory(frame, &chunks[1], self.current_directory.as_mut().unwrap());

        if let Some(directory) = &mut self.next_directory {
            App::render_directory(frame, &chunks[2], directory);
        }

        self.action = None;
    }

    fn render_directory<B: Backend>(
        frame: &mut Frame<'_, B>,
        chunk: &Rect,
        directory: &mut Directory,
    ) {
        let items: Vec<ListItem> = directory
            .entries
            .iter()
            .map(|entry| {
                let file_name = entry.0.file_name().unwrap().to_string_lossy();

                if entry.1.is_dir() {
                    ListItem::new(file_name).style(
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::BOLD),
                    )
                } else if entry.1.is_symlink() {
                    ListItem::new(file_name).style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    ListItem::new(file_name).style(
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    )
                }
            })
            .collect();

        let current_directory_block = Block::default();

        let list = List::new(items)
            .block(current_directory_block)
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_stateful_widget(list, *chunk, &mut directory.state);
    }
}
