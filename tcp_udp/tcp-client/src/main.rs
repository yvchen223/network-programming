use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8888").unwrap();

    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = vec![];
        io::stdin().read_line(&mut input).unwrap();
        stream.write(input.as_bytes()).unwrap();

        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buffer).unwrap();
        println!("{}", std::str::from_utf8(&buffer).unwrap());
    }
}
