mod render;
mod utils;

use std::fs::FileType;
use std::io;
use std::path::PathBuf;
use tui::widgets::ListState;

#[derive(Debug)]
pub struct Directory {
    permission_denied: bool,
    root: PathBuf,
    state: ListState,
    entries: Vec<(PathBuf, FileType)>,
}

impl Directory {
    pub fn new(dir_path: &PathBuf) -> Self {
        let mut state: ListState = ListState::default();
        state.select(None);

        let entries: io::Result<Vec<(PathBuf, FileType)>> = Directory::build_entries(dir_path);

        if let Ok(entries) = entries {
            Directory {
                permission_denied: false,
                root: dir_path.to_path_buf(),
                state,
                entries,
            }
        } else {
            Directory {
                permission_denied: true,
                root: dir_path.to_path_buf(),
                state,
                entries: Vec::new(),
            }
        }
    }

    pub fn get_root(&self) -> &PathBuf {
        &self.root
    }

    pub fn get_entries(&self) -> &Vec<(PathBuf, FileType)> {
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
