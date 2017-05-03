extern crate serial;

use serial::PortSettings;
use serial::SystemPort;
use serial::prelude::*;
use std::io::{Read, Write};
use std::time::Duration;

// change connection settings here
const SETTINGS: PortSettings = PortSettings {
    baud_rate: serial::Baud9600,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowHardware,
};

pub struct PrinterDriver {
    port: SystemPort,
}

impl PrinterDriver {
    pub fn new(portname: String) -> Self {
        let mut port = serial::open(portname.as_str())
            .unwrap_or_else(|_| panic!("Can't open port {}", portname));
        port.configure(&SETTINGS).unwrap();
        port.set_timeout(Duration::from_millis(1000 * 60)).unwrap();    // 60 seconds

        PrinterDriver { port: port }
    }

    pub fn send(&mut self, command: String) {
        print!("{}", command);
        self.port.write(&command.into_bytes()).unwrap();
        self.port.flush().unwrap();
    }

    pub fn wait_to_response(&mut self) -> String {
        let mut buf = String::new();
        let mut char_buf: Vec<u8> = vec![0];
        while char_buf[0] as char != '\n' {
            self.port.read(&mut char_buf[..]).unwrap_or_else(|_| panic!("Read time out"));
            buf.push(char_buf[0] as char);
        }
        print!("{}", buf);
        buf
    }
}