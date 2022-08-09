use std::fs::{self, DirEntry, FileType};
use std::path::PathBuf;
use tui::widgets::ListState;

#[derive(Debug)]
pub struct Directory {
    pub root: PathBuf,
    pub state: ListState,
    pub entries: Vec<(PathBuf, FileType)>,
}

impl Directory {
    pub fn new(dir_path: &PathBuf) -> std::io::Result<Self> {
        let mut entries: Vec<(PathBuf, FileType)> = Vec::new();

        for entry in fs::read_dir(dir_path)? {
            let entry: DirEntry = entry?;
            entries.push((entry.path(), entry.file_type()?));
        }

        let mut state: ListState = ListState::default();
        state.select(None);

        Ok(Directory {
            root: dir_path.to_path_buf(),
            state,
            entries,
        })
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
}
