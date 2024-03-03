use embedded_nano_mesh::{AddressType, LifeTimeType, Node, NodeConfig, NodeString, SendError};

use platform_millis_linux::{ms, LinuxTime};
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
        device_address: 1 as AddressType,
        listen_period: 150 as ms,
    });

    match mesh_node.send(
        NodeString::from("This is the message to be sent").into_bytes(),
        2 as AddressType,
        10 as LifeTimeType,
        true,
    ) {
        Ok(()) => {
            ufmt::uwriteln!(&mut LinuxSerial::default(), "Packet sent").unwrap();
        }
        Err(SendError::SendingQueueIsFull) => {
            ufmt::uwriteln!(&mut LinuxSerial::default(), "SendingQueueIsFull").unwrap();
        }
    }

    loop {
        let _ = mesh_node.update::<LinuxTime, LinuxSerial>();
    }
}
