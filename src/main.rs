use embedded_nano_mesh::*;

use platform_millis_linux::{ms, LinuxTime, PlatformTime};
use platform_serial_linux::{
    configure_serial, BaudRate, CharSize, FlowControl, LinuxSerial, Parity, PortSettings, StopBits,
};

fn main() {
    configure_serial(
        "/dev/ttyUSB0".to_string(),
        PortSettings {
            baud_rate: BaudRate::Baud9600,
            char_size: CharSize::Bits8,
            parity: Parity::ParityNone,
            stop_bits: StopBits::Stop1,
            flow_control: FlowControl::FlowNone,
        },
    );

    let mut node = Node::new(NodeConfig {
        device_address: 1 as AddressType,
        listen_period: 150 as ms,
    });

    let exit_time = LinuxTime::millis() + 300 as ms;

    let _ = node.send(
        NodeString::from("beep").into_bytes(),
        3 as AddressType,
        12 as LifeTimeType,
        true,
    );

    loop {
        let current_time = LinuxTime::millis();

        if current_time >= exit_time {
            break;
        }
        let _ = node.update::<LinuxTime, LinuxSerial>();
    }
}
