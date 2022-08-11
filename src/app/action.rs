use crate::app::{Action, App};

impl App {
    pub fn handle_action(&mut self) {
        match self.action {
            Some(Action::Left) => {
                if self.previous_directory.is_some() {
                    self.next_directory = self.current_directory.take();
                    self.next_directory.as_mut().unwrap().state.select(None);

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
                    if self.current_directory.as_ref().unwrap().entries.len() > 0 {
                        self.current_directory
                            .as_mut()
                            .unwrap()
                            .state
                            .select(Some(0));
                    }

                    self.build_next_dir();
                }
            }
            Some(Action::Up) => {
                self.current_directory.as_mut().unwrap().previous();
                self.build_next_dir();
            }
            Some(Action::Down) => {
                self.current_directory.as_mut().unwrap().next();
                self.build_next_dir();
            }
            _ => {}
        }
    }
}
