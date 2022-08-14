use crate::app::directory::Directory;
use std::fs::FileType;
use std::fs::{self, DirEntry};
use std::path::PathBuf;

impl Directory {
    pub fn build_entries(dir_path: &PathBuf) -> std::io::Result<Vec<(PathBuf, FileType)>> {
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
}
