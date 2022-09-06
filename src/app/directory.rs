mod render;
mod utils;

use crate::app::entry::Entry;
use std::path::PathBuf;
use tui::widgets::ListState;

#[derive(Debug)]
pub struct Directory {
    permission_denied: bool,
    root: PathBuf,
    state: ListState,
    visible_entries: usize,
    entries: Vec<Entry>,
}

impl Directory {
    pub fn new(dir_path: &PathBuf) -> Self {
        let mut state: ListState = ListState::default();
        state.select(None);

        let entries: std::io::Result<Vec<Entry>> = Directory::build_entries(dir_path);

        match entries {
            Ok(entries) => {
                let visible_entries: usize = entries
                    .iter()
                    .filter(|entry: &&Entry| entry.is_visible())
                    .count();

                Directory {
                    permission_denied: false,
                    root: dir_path.to_path_buf(),
                    state,
                    entries,
                    visible_entries,
                }
            }
            Err(error) => {
                if error.kind() == std::io::ErrorKind::PermissionDenied {
                    Directory {
                        permission_denied: true,
                        root: dir_path.to_path_buf(),
                        state,
                        entries: Vec::new(),
                        visible_entries: 0,
                    }
                } else {
                    panic!(
                        "An error occured reading the directory {}: {}",
                        dir_path.display(),
                        error
                    );
                }
            }
        }
    }

    pub fn get_root(&self) -> &PathBuf {
        &self.root
    }

    pub fn get_entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.len() == 0
    }

    pub fn set_state(&mut self, index: Option<usize>) {
        self.state.select(index);
    }

    pub fn get_state(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn is_permission_denied(&self) -> bool {
        self.permission_denied
    }
}
