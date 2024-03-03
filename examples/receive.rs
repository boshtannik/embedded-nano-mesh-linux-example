use embedded_nano_mesh::{AddressType, Node, NodeConfig};

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

    loop {
        let _ = mesh_node.update::<LinuxTime, LinuxSerial>();
        if let Some(packet) = mesh_node.receive() {
            ufmt::uwriteln!(
                &mut LinuxSerial::default(),
                "Packet from: {}",
                packet.source_device_identifier
            )
            .unwrap();

            for character in packet.data {
                let character = char::from(character);
                ufmt::uwrite!(&mut LinuxSerial::default(), "{}", character).unwrap();
            }
            ufmt::uwriteln!(&mut LinuxSerial::default(), "").unwrap();
        }
    }
}
