/// Functions for communication with the modem
mod comm;

/// Specific errors for the crate
mod error;

#[cfg(test)]
mod tests {
    use crate::comm::Modem;

    #[test]
    fn test_connection() {
        let mut modem = Modem::new("/dev/ttyACM0");
        match modem.check_connection() {
            Ok(_) => println!("Modem connected OK"),
            Err(e) => panic!("Failed to connect to modem: {}", e),
        }
    }

    #[test]
    fn test_manufacturer() {
        let mut modem = Modem::new("/dev/ttyACM0");
        match modem.check_connection() {
            Ok(_) => println!("Modem connected OK"),
            Err(e) => panic!("Failed to connect to modem: {}", e),
        }
    }
}