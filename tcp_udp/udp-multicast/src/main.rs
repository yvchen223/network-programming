use std::env;
use std::net::{Ipv4Addr, UdpSocket};

// Step1: cargo run -- server
// Step2: cargo run
fn main() {
    let mcast_group: Ipv4Addr = "239.0.0.1".parse().unwrap();
    let port: u16 = 6000;
    let any = "0.0.0.0".parse().unwrap();
    let mut buffer = [0_u8; 1600];
    if env::args().count() > 1 {
        // server
        let socket = UdpSocket::bind((any, port)).unwrap();
        socket.join_multicast_v4(&mcast_group, &any).unwrap();
        socket.recv_from(&mut buffer).unwrap();
        println!("receive {}", std::str::from_utf8(&buffer).unwrap());
    } else {
        // client
        let socket = UdpSocket::bind((any, 0)).unwrap();
        socket.send_to("Hello world!".as_bytes(), &(mcast_group, port)).unwrap();

    }
}
