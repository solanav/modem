use crate::error::Error;
use serial::prelude::*;
use serial::SerialPort;
use std::time::Duration;

/// Different states the modem can be in
pub enum State {
    Disconnected,
    Connected,
}

/// Mantains data about a modem connection
pub struct Modem {
    port: Box<SerialPort>,
    state: State,
}

impl Modem {
    pub fn new(location: &'static str) -> Self {
        Modem {
            port: Box::new(serial::open(location).unwrap()),
            state: State::Disconnected,
        }
    }

    pub fn check_connection(&mut self) -> Result<(), Error> {
        self.port.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud9600).unwrap();
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        }).unwrap();

        self.port.set_timeout(Duration::from_millis(1000));

        self.port.write("AT\r\n".as_bytes()).unwrap();

        let mut buf: Vec<u8> = vec![0; 5];
        self.port.read(&mut buf).unwrap();

        match buf {

        }

        println!("port > [{}]", String::from_utf8(buf).unwrap());

        Ok(())
    }
}