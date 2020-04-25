use crate::error::Error;
use serial::SerialPort;
use std::time::Duration;
use crate::error::Error::CheckConnection;
use crate::comm::State::Connected;

/// Different states the modem can be in
pub enum State {
    Disconnected,
    Connected,
}

/// Mantains data about a modem connection
pub struct Modem {
    port: Box<dyn SerialPort>,
    state: State,
}

impl Modem {
    pub fn new(location: &'static str) -> Self {
        let mut port = serial::open(location).unwrap();

        port.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud9600).unwrap();
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        }).unwrap();

        port.set_timeout(Duration::from_millis(20000));

        Modem {
            port: Box::new(port),
            state: State::Disconnected,
        }
    }

    pub fn check_connection(&mut self) -> Result<(), Error> {
        self.port.write("AT\r\n".as_bytes()).unwrap();

        let mut buf = String::new();
        self.port.read_to_string(&mut buf).unwrap();

        if buf == "\r\nOK\r".to_string() {
            return Ok(())
        }

        Err(CheckConnection())
    }

    pub fn get_manufacturer(&mut self) -> Result<(), Error> {
        self.port.write("AT+CGMI\r\n".as_bytes()).unwrap();

        let mut buf= String::new();
        self.port.read_to_string(&mut buf).unwrap();
        println!("[{}]", buf);

        Err(CheckConnection())
    }
}