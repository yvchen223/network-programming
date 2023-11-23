use std::io;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8000").unwrap();
    socket.connect("127.0.0.1:8888").unwrap();

    loop {
        let mut input = String::new();
        let mut buffer = [0_u8; 1500];
        io::stdin().read_line(&mut input).unwrap();

        socket.send(input.as_bytes()).unwrap();

        socket.recv_from(&mut buffer).unwrap();
        println!("receive: {}", std::str::from_utf8(&buffer).unwrap());
    }
}
