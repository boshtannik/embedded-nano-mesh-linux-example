use embedded_nano_mesh::{AddressType, LifeTimeType, Node, NodeConfig, NodeString};

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

    let mut packet_sent_counter: u32 = 0;

    loop {
        let mut message = NodeString::new();
        ufmt::uwrite!(message, "Packet: #{}", packet_sent_counter).unwrap();

        match mesh_node.send_ping_pong::<LinuxTime, LinuxSerial>(
            message.clone().into_bytes(),
            2 as AddressType,
            10 as LifeTimeType,
            true,
            3000 as ms,
        ) {
            Ok(()) => {
                ufmt::uwriteln!(&mut LinuxSerial::default(), "Pingpong sent").unwrap();
            }
            Err(_) => ufmt::uwriteln!(&mut LinuxSerial::default(), "Pingpong failed").unwrap(),
        }
        packet_sent_counter += 1;

        let _ = mesh_node.update::<LinuxTime, LinuxSerial>();
    }
}
