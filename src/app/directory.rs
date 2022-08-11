use std::fs::{self, DirEntry, FileType};
use std::io;
use std::path::PathBuf;
use tui::widgets::ListState;

#[derive(Debug)]
pub struct Directory {
    permission_denied: bool,
    pub root: PathBuf,
    pub state: ListState,
    pub entries: Vec<(PathBuf, FileType)>,
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

    fn build_entries(dir_path: &PathBuf) -> std::io::Result<Vec<(PathBuf, FileType)>> {
        let mut entries: Vec<(PathBuf, FileType)> = Vec::new();

        for entry in fs::read_dir(dir_path)? {
            let entry: DirEntry = entry?;
            entries.push((entry.path(), entry.file_type()?));
        }

        Ok(entries)
    }

    pub fn next(&mut self) {
        let i: usize = match self.state.selected() {
            Some(i) => {
                if i >= self.entries.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i: usize = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.entries.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
    }

    pub fn is_empty(&self) -> bool {
        self.entries.len() == 0
    }
}
