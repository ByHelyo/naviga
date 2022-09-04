use crate::app::App;

use crate::app::{directory::Directory, entry::Entry};
use std::path::PathBuf;

impl App {
    pub fn build_previous_dir(&mut self) {
        let current_path: &PathBuf = self.current_directory.as_ref().unwrap().get_root();

        match current_path.parent() {
            Some(parent) => {
                // Build previous directory
                self.previous_directory = Some(Directory::new(&parent.to_path_buf()));

                // Get the index of the parent in previous directory
                let parent_index: usize = self
                    .previous_directory
                    .as_ref()
                    .unwrap()
                    .get_entry_from_path(current_path)
                    .unwrap();

                self.previous_directory
                    .as_mut()
                    .unwrap()
                    .set_state(Some(parent_index));
            }
            None => {
                self.previous_directory = None;
            }
        }
    }

    pub fn build_next_dir(&mut self) {
        let current_directory: &Directory = self.current_directory.as_ref().unwrap();

        if !current_directory.is_empty() {
            let current_entry: &Entry =
                current_directory.nth_visible(current_directory.get_state().unwrap());

            if current_entry.is_dir() {
                self.next_directory = Some(Directory::new(current_entry.get_path()));
            } else {
                self.next_directory = None;
            }
        }
    }
}
