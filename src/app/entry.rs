use std::borrow::Cow;
use std::fs::{DirEntry, FileType};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Entry {
    file_type: FileType,
    path: PathBuf,
    visible: bool,
}

impl Entry {
    pub fn new(entry: &DirEntry) -> Self {
        let mut visible: bool = true;
        let path: PathBuf = entry.path();

        if path.file_name().unwrap().to_string_lossy().starts_with('.') {
            visible = false;
        }

        Entry {
            file_type: entry.file_type().unwrap(),
            path,
            visible,
        }
    }

    pub fn is_symlink(&self) -> bool {
        self.file_type.is_symlink()
    }

    pub fn is_dir(&self) -> bool {
        self.file_type.is_dir()
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_file_type(&self) -> &FileType {
        &self.file_type
    }

    pub fn get_filename(&self) -> Cow<str> {
        self.path.file_name().unwrap().to_string_lossy()
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}
