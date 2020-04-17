use std::path::Path;
use std::fs::File;
use std::io::Write;

/// Different states the modem can be in
pub enum State {
    Disconnected,
    Connected,
}

/// Mantains data about a modem connection
pub struct Modem {
    file: File,
    state: State,
}

impl Modem {
    pub fn new(location: &Path) -> Self {
        Modem {
            file: File::open(location).unwrap(),
            state: State::Disconnected,
        }
    }

    pub fn check_connection(&mut self) {
        self.file.write("AT\r\n".as_bytes());
    }
}