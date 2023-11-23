use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// Start a terminal and type `nc 0.0.0.0 8888` as a client.
fn main() {
    let listen = TcpListener::bind("0.0.0.0:8888").unwrap();

    for stream in listen.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle(stream).unwrap_or_else(|e| eprintln!("{:?}", e));
                });
            },
            Err(e) => eprintln!("failed: {:?}", e)
        }
    }
}

fn handle(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            break;
        }
        stream.write(&buf[..bytes_read])?;
    }
    Ok(())
}
