use pcap::{Capture, Device};

fn main() {
    // Find available network devices
    let devices = Device::list().expect("Failed to get devices");
    let device = devices.into_iter().next().expect("No devices found");

    println!("Listening on device: {}", device.name);

    // Open device in promiscuous mode (captures all packets)
    let mut cap = Capture::from_device(device)
        .unwrap()
        .promisc(true)  // Capture all traffic
        .snaplen(65535) // Max packet size
        .open()
        .unwrap();

    // Capture packets in a loop
    while let Ok(packet) = cap.next_packet() {
        println!("Captured packet: {} bytes", packet.header.len);
    }
}
