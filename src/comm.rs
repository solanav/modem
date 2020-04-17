/// Different states the modem can be in
pub enum State {
    Disconnected,
    Connected,
}

/// Mantains data about a modem connection
pub struct Modem {
    location: Path,
    state: State,
}

impl Modem {
    pub fn new(location: Path) -> Self {
        Modem {
            location,
            state: State::Disconnected,
        }
    }
}