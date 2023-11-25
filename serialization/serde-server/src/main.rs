use std::{env, thread};
use std::io::{BufRead, BufReader, Error, stdin, Write};
use std::net::{TcpListener, TcpStream};
use serde_derive::{Deserialize, Serialize};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Invalid args.");
        std::process::exit(1);
    }
    if args[1] == "--server" {
        let listener = TcpListener::bind("0.0.0.0:8888").unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        handle_client(stream).unwrap_or_else(|e| eprintln!("{:?}", e));
                    });
                },
                Err(e) => eprintln!("failed: {}", e),
            }
        }
    } else if args[1] == "--client" {
        let mut stream = TcpStream::connect("127.0.0.1:8888").unwrap();
        println!("Please input three comma separated integers");
        loop {
            let mut input = String::new();
            let mut buffer: Vec<u8> = vec![];
            stdin().read_line(&mut input).unwrap();
            let parts = input.trim_matches('\n').split(',').collect::<Vec<&str>>();

            let point = Point3D {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
                z: parts[2].parse().unwrap(),
            };
            stream.write_all(serde_json::to_string(&point).unwrap().as_bytes()).unwrap();
            stream.write_all(b"\n").unwrap();

            let mut reader = BufReader::new(&stream);
            reader.read_until(b'\n', &mut buffer).unwrap();

            let input  = std::str::from_utf8(&buffer).unwrap();
            if input == "" {
                eprintln!("Empty response");
            }
            println!("Response {}", input);
        }
    }
    println!("Hello, world!");
}

#[derive(Serialize, Deserialize, Debug)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

fn handle_client(stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut data = vec![];
    let mut stream = BufReader::new(stream);

    loop {
        data.clear();
        let bytes_read = stream.read_until(b'\n', &mut data)?;
        if bytes_read == 0 {
            return Ok(());
        }
        let input: Point3D = serde_json::from_slice(&data)?;
        println!("get point {:?}", input);
        let value = input.x.pow(2) + input.y.pow(2) + input.z.pow(2);
        write!(stream.get_mut(), "{}", f64::from(value).sqrt())?;
        write!(stream.get_mut(), "{}", "\n")?;
    }
}
