use embedded_nano_mesh::{ms, ExactAddressType, Node, NodeConfig};
use serialport;
use std::time::Instant;
mod serial_driver;
use serial_driver::*;

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
