use crate::app::{directory::Directory, entry::Entry};
use std::cmp::Ordering;
use std::fs::{self, DirEntry};
use std::path::PathBuf;

impl Directory {
    pub fn build_entries(dir_path: &PathBuf) -> std::io::Result<Vec<Entry>> {
        let mut entries: Vec<Entry> = Vec::new();

        for entry in fs::read_dir(dir_path)? {
            let entry: DirEntry = entry?;
            entries.push(Entry::new(&entry));
        }

        entries.sort_by(|a: &Entry, b: &Entry| {
            if a.get_file_type().is_dir() {
                if !b.get_file_type().is_dir() {
                    return Ordering::Less;
                }

                return a.get_path().cmp(b.get_path());
            }

            if b.get_file_type().is_dir() {
                return Ordering::Greater;
            }

            a.get_path().cmp(b.get_path())
        });

        Ok(entries)
    }

    pub fn get_selected_entry(&self) -> Option<&Entry> {
        if let Some(index) = self.get_state() {
            let entry: &Entry = self
                .entries
                .iter()
                .filter(|entry: &&Entry| entry.is_visible())
                .nth(index)
                .unwrap();

            Some(entry)
        } else {
            None
        }
    }

    pub fn get_entry_from_path(&self, path: &PathBuf) -> Option<usize> {
        for (index, entry) in self
            .entries
            .iter()
            .filter(|entry: &&Entry| entry.is_visible())
            .enumerate()
        {
            if entry.is_dir() && entry.get_path() == path {
                return Some(index);
            }
        }
        None
    }

    pub fn nth_visible(&self, nth: usize) -> &Entry {
        self.entries
            .iter()
            .filter(|entry: &&Entry| entry.is_visible())
            .nth(nth)
            .unwrap()
    }

    pub fn next(&mut self) {
        let i: usize = match self.state.selected() {
            Some(i) => {
                if i >= self.visible_entries - 1 {
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
                    self.visible_entries - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
    }
}
