mod action;
mod directory;
mod utils;

use directory::Directory;
use std::borrow::Cow;
use std::path::PathBuf;
use std::{env, error};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, List, ListItem, Paragraph},
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

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

        App::render_current_directory_path(
            frame,
            &main_chunks[0],
            self.current_directory.as_ref().unwrap(),
        );

        if let Some(directory) = &mut self.previous_directory {
            App::render_directory(frame, &directories_chunks[0], directory);
        }

        App::render_directory(
            frame,
            &directories_chunks[2],
            self.current_directory.as_mut().unwrap(),
        );

        if let Some(directory) = &mut self.next_directory {
            App::render_directory(frame, &directories_chunks[4], directory);
        }

        self.action = None;
    }

    fn render_current_directory_path<B: Backend>(
        frame: &mut Frame<'_, B>,
        chunk: &Rect,
        current_directory: &Directory,
    ) {
        let current_path: Paragraph = Paragraph::new(Spans::from(vec![
            Span::styled(
                "current directory: ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                current_directory.get_root().to_str().unwrap(),
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        frame.render_widget(current_path, *chunk);
    }

    fn render_directory<B: Backend>(
        frame: &mut Frame<'_, B>,
        chunk: &Rect,
        directory: &mut Directory,
    ) {
        if directory.is_permission_denied() {
            let paragraph: Paragraph = Paragraph::new(Span::styled(
                "Permission denied",
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ))
            .block(Block::default());

            frame.render_widget(paragraph, *chunk);
            return;
        } else if directory.is_empty() {
            let paragraph = Paragraph::new(Span::styled(
                "Empty",
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ))
            .block(Block::default());

            frame.render_widget(paragraph, *chunk);

            return;
        }

        let items: Vec<ListItem> = directory
            .entries
            .iter()
            .map(|entry| {
                let file_name: Cow<'_, str> = entry.0.file_name().unwrap().to_string_lossy();
                let file_name = Spans::from(vec![Span::raw(" "), Span::raw(file_name)]);

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

        let list: List;

        if let Some(index) = directory.get_state() {
            if directory.get_entries()[index].1.is_dir() {
                list = List::new(items)
                    .block(current_directory_block)
                    .highlight_style(
                        Style::default()
                            .bg(Color::Blue)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    );
            } else if directory.get_entries()[index].1.is_symlink() {
                list = List::new(items)
                    .block(current_directory_block)
                    .highlight_style(
                        Style::default()
                            .bg(Color::Cyan)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    );
            } else {
                list = List::new(items)
                    .block(current_directory_block)
                    .highlight_style(
                        Style::default()
                            .bg(Color::White)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    );
            }
        } else {
            list = List::new(items).block(current_directory_block);
        }

        frame.render_stateful_widget(list, *chunk, &mut directory.state);
    }
}
