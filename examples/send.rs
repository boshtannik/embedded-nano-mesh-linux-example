use embedded_nano_mesh::{
    ExactAddressType, GeneralAddressType, LifeTimeType, Node, NodeConfig, NodeString, SendError,
};

use platform_millis_linux::{ms, LinuxMillis};
use platform_serial_linux::{
    configure_serial, CharSize, FlowControl, LinuxSerial, Parity, PortSettings, StopBits,
};

fn main() -> ! {
    configure_serial(
        "/dev/ttyUSB0".to_string(),
        PortSettings {
            baud_rate: serial_core::BaudRate::Baud9600,
            char_size: CharSize::Bits8,
            parity: Parity::ParityNone,
            stop_bits: StopBits::Stop1,
            flow_control: FlowControl::FlowNone,
        },
    );

    let mut mesh_node = Node::new(NodeConfig {
        device_address: ExactAddressType::new(1).unwrap(),
        listen_period: 150 as ms,
    });

    match mesh_node.send(
        NodeString::from("This is the message to be sent").into_bytes(),
        GeneralAddressType::Exact(ExactAddressType::new(2).unwrap()),
        10 as LifeTimeType,
        true,
    ) {
        Ok(()) => {
            println!("Packet sent");
        }
        Err(SendError::SendingQueueIsFull) => {
            println!("Sending queue is full");
        }
    }

    loop {
        let _ = mesh_node.update::<LinuxMillis, LinuxSerial>();
    }
}
