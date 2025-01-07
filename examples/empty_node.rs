use embedded_nano_mesh::{ms, ExactAddressType, Node, NodeConfig};
use serialport;
use std::{
    io::{Read, Write},
    time::Instant,
};
struct LinuxInterfaceDriver {
    serial: serialport::TTYPort,
}

impl LinuxInterfaceDriver {
    pub fn new(serial: serialport::TTYPort) -> LinuxInterfaceDriver {
        LinuxInterfaceDriver { serial }
    }
}

impl embedded_serial::MutBlockingTx for LinuxInterfaceDriver {
    type Error = ();

    fn putc(&mut self, ch: u8) -> Result<(), Self::Error> {
        self.serial.write(&[ch]).unwrap();
        Ok(())
    }
}

impl embedded_serial::MutNonBlockingRx for LinuxInterfaceDriver {
    type Error = ();

    fn getc_try(&mut self) -> Result<Option<u8>, Self::Error> {
        let mut buf = [0u8];
        match self.serial.read(&mut buf) {
            Ok(_) => Ok(Some(buf[0])),
            Err(_) => Ok(None),
        }
    }
}

fn main() -> ! {
    let program_start_time = Instant::now();

    let mut serial = LinuxInterfaceDriver::new(
        serialport::new("/dev/ttyUSB0", 9600)
            .open_native()
            .expect("Fail to open serial port"),
    );

    let mut mesh_node = Node::new(NodeConfig {
        device_address: ExactAddressType::new(2).unwrap(),
        listen_period: 150 as ms,
    });

    loop {
        let current_time = Instant::now()
            .duration_since(program_start_time)
            .as_millis() as ms;

        let _ = mesh_node.update(&mut serial, current_time);
    }
}
