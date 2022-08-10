use crate::app::Action;
use crate::app::App;

impl App {
    pub fn handle_action(&mut self) {
        match self.action {
            Some(Action::Left) => {
                if self.previous_directory.is_some() {
                    self.next_directory = self.current_directory.take();
                    self.next_directory.as_mut().unwrap().state.select(None);

                    self.current_directory = self.previous_directory.take();

                    self.build_previous_dir().unwrap();
                }
            }
            Some(Action::Right) => {
                if self.next_directory.is_some() {
                    self.previous_directory = self.current_directory.take();

                    self.current_directory = self.next_directory.take();
                    self.current_directory
                        .as_mut()
                        .unwrap()
                        .state
                        .select(Some(0));

                    self.build_next_dir().unwrap();
                }
            }
            Some(Action::Up) => {
                self.current_directory.as_mut().unwrap().previous();
                if let Err(error) = self.build_next_dir() {
                    if let std::io::ErrorKind::PermissionDenied = error.kind() {
                        self.next_directory = None;
                    } else {
                        panic!("{}", error);
                    }
                }
            }
            Some(Action::Down) => {
                self.current_directory.as_mut().unwrap().next();
                if let Err(error) = self.build_next_dir() {
                    if let std::io::ErrorKind::PermissionDenied = error.kind() {
                        self.next_directory = None;
                    } else {
                        panic!("{}", error);
                    }
                }
            }
            _ => {}
        }
    }
}