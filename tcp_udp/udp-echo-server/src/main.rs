use std::net::UdpSocket;
use std::thread;

// Start a terminal and type `nc -u 127.0.0.1 8888` as a client.
fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").unwrap();

    loop {
        let mut buf = [0_u8; 1500];
        let sock = socket.try_clone().unwrap();
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    println!("Handling connection from {}", src);
                    sock.send_to(&buf, &src).unwrap();
                });
            },
            Err(e) => eprintln!("couldn't receive: {}", e),
        }
    }
}
