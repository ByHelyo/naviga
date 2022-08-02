use std::error;
use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, Paragraph};

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
    pub key: Option<Action>,
}

impl Default for App {
    fn default() -> Self {
        Self { running: true, key: None }
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

        match self.key {
            Some(Action::Left) => {
                frame.render_widget(
                    Paragraph::new("left")
                        .block(Block::default().borders(Borders::ALL))
                        .style(Style::default().fg(Color::White).bg(Color::Black))
                        .alignment(Alignment::Center),
                    frame.size(),
                );
            }
            Some(Action::Right) => {
                frame.render_widget(
                    Paragraph::new("Right")
                        .block(Block::default().borders(Borders::ALL))
                        .style(Style::default().fg(Color::White).bg(Color::Black))
                        .alignment(Alignment::Center),
                    frame.size(),
                );
            }
            Some(Action::Up) => {
                frame.render_widget(
                    Paragraph::new("Up")
                        .block(Block::default().borders(Borders::ALL))
                        .style(Style::default().fg(Color::White).bg(Color::Black))
                        .alignment(Alignment::Center),
                    frame.size(),
                );
            }
            Some(Action::Down) => {
                frame.render_widget(
                    Paragraph::new("Down")
                        .block(Block::default().borders(Borders::ALL))
                        .style(Style::default().fg(Color::White).bg(Color::Black))
                        .alignment(Alignment::Center),
                    frame.size(),
                );
            }
            _ => {
                frame.render_widget(
                    Paragraph::new("naviga")
                        .block(Block::default().borders(Borders::ALL))
                        .style(Style::default().fg(Color::White).bg(Color::Black))
                        .alignment(Alignment::Center),
                    frame.size(),
                );
            }
        }
    }
}
