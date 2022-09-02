use crate::app::{Action, App};
use std::fs::File;
use std::io::Write;

impl App {
    pub fn handle_action(&mut self) {
        match self.action {
            Some(Action::Left) => {
                if self.previous_directory.is_some() {
                    self.next_directory = self.current_directory.take();
                    self.next_directory.as_mut().unwrap().set_state(None);

                    self.current_directory = self.previous_directory.take();

                    self.build_previous_dir();
                }
            }
            Some(Action::Right) => {
                if self.next_directory.is_some() {
                    self.previous_directory = self.current_directory.take();

                    self.current_directory = self.next_directory.take();

                    // if current directory is not empty
                    // Set the list state index to 0
                    if !self.current_directory.as_ref().unwrap().is_empty() {
                        self.current_directory.as_mut().unwrap().set_state(Some(0));
                    }

                    self.build_next_dir();
                }
            }
            Some(Action::Up) => {
                if !self.current_directory.as_ref().unwrap().is_empty() {
                    self.current_directory.as_mut().unwrap().previous();
                    self.build_next_dir();
                }
            }
            Some(Action::Down) => {
                if !self.current_directory.as_ref().unwrap().is_empty() {
                    self.current_directory.as_mut().unwrap().next();
                    self.build_next_dir();
                }
            }
            Some(Action::Enter) => {
                let mut file = File::create(dirs::home_dir().unwrap().join("naviga.sh")).unwrap();
                write!(
                    file,
                    "cd \"{}\"",
                    self.current_directory
                        .as_ref()
                        .unwrap()
                        .get_root()
                        .to_str()
                        .unwrap()
                )
                .unwrap();
                self.running = false;
            }
            _ => {}
        }
    }
}
