/// Functions for communication with the modem
mod comm;

#[cfg(test)]
mod tests {
    use crate::comm::Modem;
    use std::path::Path;

    #[test]
    fn it_works() {
        let modem = Modem::new(Path::new("/dev/ttyACM0"));
    }
}
