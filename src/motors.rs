use communication::{Port, SerialConnection, Message};

const COMMAND_MOTORS_RUN: u8 = 10;
const COMMAND_MOTORS_STOP: u8 = 11;

pub struct Motors(SerialConnection);

impl Motors {
    pub fn init() -> Motors {
        let port = Port::new("/dev/serial0");
        let serial = SerialConnection::open(&port).unwrap();
        Motors(serial)
    }

    pub fn motors_stop(&mut self) {
        let mut buf = Message::new();
        buf.queue_byte(COMMAND_MOTORS_STOP);
        buf.create();
        self.0.send_data(&buf).unwrap();
    }

    pub fn motors_write(&mut self, speed_left: i16, speed_right: i16) {
        let mut buf = Message::new();
        buf.queue_byte(COMMAND_MOTORS_RUN);
        buf.queue_int(speed_left);
        buf.queue_int(speed_right);
        buf.create();
        self.0.send_data(&buf).unwrap();
    }

    pub fn turn_left(&mut self, speed: i16) {
        self.motors_write(speed, 255);

    }

    pub fn turn_right(&mut self, speed: i16) {
        self.motors_write(255, speed);
    }
}
