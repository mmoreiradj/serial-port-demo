use std::time::Duration;

use api::{CloudletMessage, CloudletProtocol};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    serial_path: String,
}

fn main() {
    let args = Args::parse();

    let port = serialport::new(args.serial_path, 115_200)
        .timeout(Duration::from_secs(10))
        .open_native()
        .expect("Failed to open serial port");

    let mut cloudlet_protocol = CloudletProtocol::new(port);

    let request = cloudlet_protocol
        .read_message()
        .expect("Failed to read message from serial port");

    println!("{:?}", request);

    let response = CloudletMessage::new(1, "yes daddy".to_string());

    cloudlet_protocol.send_message(response);
}
