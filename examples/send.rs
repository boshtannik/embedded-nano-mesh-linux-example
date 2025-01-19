use embedded_nano_mesh::{
    ms, ExactAddressType, LifeTimeType, Node, NodeConfig, NodeString, SendError,
};
use embedded_nano_mesh_linux_io::*;
use std::time::Instant;

fn main() -> ! {
    let program_start_time = Instant::now();

    let mut serial = LinuxIO::new(
        serialport::new("/dev/ttyUSB0", 9600)
            .open_native()
            .expect("Fail to open serial port"),
    );

    let mut mesh_node = Node::new(NodeConfig {
        device_address: ExactAddressType::new(1).unwrap(),
        listen_period: 150 as ms,
    });

    match mesh_node.send_to_exact(
        NodeString::from_iter("This is the message to be sent".chars()).into_bytes(), // Content.
        ExactAddressType::new(2).unwrap(), // Send to device with address 2.
        10 as LifeTimeType,                // Let message travel 10 devices before being destroyed.
        true,
    ) {
        Ok(()) => println!("Message sent"),
        Err(SendError::SendingQueueIsFull) => println!("SendingQueueIsFull"),
    }
    loop {
        let _ = mesh_node.update(
            &mut serial,
            Instant::now()
                .duration_since(program_start_time)
                .as_millis() as ms,
        );
    }
}
