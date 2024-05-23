use embedded_nano_mesh::{
    ExactAddressType, LifeTimeType, Node, NodeConfig, NodeString, SpecialSendError,
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

    match mesh_node.send_ping_pong::<LinuxMillis, LinuxSerial>(
        NodeString::from("This is the message to be sent").into_bytes(),
        ExactAddressType::new(1).unwrap(),
        10 as LifeTimeType,
        3000 as ms,
    ) {
        Ok(()) => {
            println!("Packet sent");
        }
        Err(SpecialSendError::SendingQueueIsFull) => {
            println!("Sending queue is full");
        }
        Err(SpecialSendError::Timeout) => {
            println!("Timeout");
        }
    }

    loop {
        let _ = mesh_node.update::<LinuxMillis, LinuxSerial>();
    }
}
