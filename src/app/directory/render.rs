use crate::app::{directory::Directory, entry::Entry};
use std::borrow::Cow;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, List, ListItem, Paragraph},
};

impl Directory {
    pub fn render_current_directory_path<B: Backend>(
        &self,
        frame: &mut Frame<'_, B>,
        chunk: &Rect,
    ) {
        let current_path: Paragraph = Paragraph::new(Spans::from(vec![
            Span::styled(
                "current directory: ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                self.root.to_str().unwrap(),
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        frame.render_widget(current_path, *chunk);
    }

    pub fn render_directory<B: Backend>(&mut self, frame: &mut Frame<'_, B>, chunk: &Rect) {
        if self.is_permission_denied() {
            self.render_permission_denied(frame, chunk);
        } else if self.is_empty() {
            self.render_empty_dir(frame, chunk);
        } else {
            self.render_normal_dir(frame, chunk);
        }
    }

    fn render_permission_denied<B: Backend>(&mut self, frame: &mut Frame<'_, B>, chunk: &Rect) {
        let paragraph: Paragraph = Paragraph::new(Span::styled(
            "Permission denied",
            Style::default()
                .fg(Color::White)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD),
        ))
        .block(Block::default());

        frame.render_widget(paragraph, *chunk);
    }

    fn render_empty_dir<B: Backend>(&mut self, frame: &mut Frame<'_, B>, chunk: &Rect) {
        let paragraph = Paragraph::new(Span::styled(
            "Empty",
            Style::default()
                .fg(Color::White)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD),
        ))
        .block(Block::default());

        frame.render_widget(paragraph, *chunk);
    }

    fn render_normal_dir<B: Backend>(&mut self, frame: &mut Frame<'_, B>, chunk: &Rect) {
        // Build the items
        let items: Vec<ListItem> = self
            .entries
            .iter()
            .filter(|entry: &&Entry| entry.is_visible())
            .map(|entry: &Entry| {
                let file_name: Cow<str> = entry.get_filename();
                let file_name = Spans::from(vec![Span::raw(" "), Span::raw(file_name)]);
                if entry.is_dir() {
                    ListItem::new(file_name).style(
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::BOLD),
                    )
                } else if entry.is_symlink() {
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

        // Render
        let current_directory_block = Block::default();

        let list: List;

        if let Some(entry) = self.get_selected_entry() {
            if entry.is_dir() {
                list = List::new(items)
                    .block(current_directory_block)
                    .highlight_style(
                        Style::default()
                            .bg(Color::Blue)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    );
            } else if entry.is_symlink() {
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

        frame.render_stateful_widget(list, *chunk, &mut self.state);
    }
}
