mod action;
pub mod directory;
mod directory_kind;
mod utils;

use directory::Directory;
use std::path::PathBuf;
use std::{env, error};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, List, ListItem, Paragraph};

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

impl Default for App {
    fn default() -> Self {
        let current_path: PathBuf = env::current_dir().unwrap();

        let mut current_directory: Directory = Directory::new(&current_path).unwrap();
        current_directory.state.select(Some(0));

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

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
            .split(frame.size());

        let directories_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                    Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(main_chunks[1]);

        self.handle_action();

        let current_path = Paragraph::new(
            self.current_directory
                .as_ref()
                .unwrap()
                .root
                .to_str()
                .unwrap(),
        )
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );
        frame.render_widget(current_path, main_chunks[0]);

        if let Some(directory) = &mut self.previous_directory {
            App::render_directory(frame, &directories_chunks[0], directory);
        }

        App::render_directory(
            frame,
            &directories_chunks[1],
            self.current_directory.as_mut().unwrap(),
        );

        if let Some(directory) = &mut self.next_directory {
            App::render_directory(frame, &directories_chunks[2], directory);
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
                            .bg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                } else if entry.1.is_symlink() {
                    ListItem::new(file_name).style(
                        Style::default()
                            .fg(Color::Cyan)
                            .bg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    ListItem::new(file_name).style(
                        Style::default()
                            .fg(Color::White)
                            .bg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                }
            })
            .collect();

        let current_directory_block = Block::default();

        let list: List;

        if let Some(index) = directory.state.selected() {
            if directory.entries[index].1.is_dir() {
                list = List::new(items)
                    .block(current_directory_block)
                    .highlight_style(
                        Style::default()
                            .bg(Color::Blue)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol("> ");
            } else if directory.entries[index].1.is_symlink() {
                list = List::new(items)
                    .block(current_directory_block)
                    .highlight_style(
                        Style::default()
                            .bg(Color::Cyan)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol("> ");
            } else {
                list = List::new(items)
                    .block(current_directory_block)
                    .highlight_style(
                        Style::default()
                            .bg(Color::White)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol("> ");
            }
        } else {
            list = List::new(items).block(current_directory_block);
        }

        frame.render_stateful_widget(list, *chunk, &mut directory.state);
    }
}
