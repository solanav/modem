/// Functions for communication with the modem
mod comm;

#[cfg(test)]
mod tests {
    use crate::comm::Modem;

    #[test]
    fn it_works() {
        let modem = Modem::new("/dev/ttyACM0");
    }
}
