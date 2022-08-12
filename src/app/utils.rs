use crate::app::App;

use crate::app::directory::Directory;
use std::fs::FileType;
use std::path::PathBuf;

impl App {
    pub fn build_previous_dir(&mut self) {
        let current_path: &PathBuf = &self.current_directory.as_ref().unwrap().root;

        match current_path.parent() {
            Some(parent) => {
                // Build previous directory
                self.previous_directory = Some(Directory::new(&parent.to_path_buf()));

                // Get the index of the parent in previous directory
                let mut parent_index: usize = 0;

                for (index, entry) in self
                    .previous_directory
                    .as_ref()
                    .unwrap()
                    .entries
                    .iter()
                    .enumerate()
                {
                    if entry.1.is_dir() && &entry.0 == current_path {
                        parent_index = index;
                    }
                }

                self.previous_directory
                    .as_mut()
                    .unwrap()
                    .state
                    .select(Some(parent_index));
            }
            None => {
                self.previous_directory = None;
            }
        }
    }

    pub fn build_next_dir(&mut self) {
        let current_directory: &Directory = &self.current_directory.as_ref().unwrap();
        let current_entries: &Vec<(PathBuf, FileType)> =
            &self.current_directory.as_ref().unwrap().entries;

        if !current_directory.entries.is_empty() {
            let current_entry: &(PathBuf, FileType) =
                &current_entries[current_directory.state.selected().unwrap()];

            if current_entry.1.is_dir() {
                self.next_directory = Some(Directory::new(&current_entry.0));
            } else {
                self.next_directory = None;
            }
        }
    }
}
