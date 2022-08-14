use crate::app::directory::Directory;
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

    pub fn render_directory<B: Backend>(
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
