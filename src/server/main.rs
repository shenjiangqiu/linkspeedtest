use std::env::args;
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let addr = args().nth(1).unwrap_or_else(|| ":::0".to_string());
    let listener = TcpListener::bind(addr).unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    if let Ok((mut stream, addr)) = listener.accept() {
        println!("Accepted connection from {}", addr);
        let mut buffer = [0; 13];
        stream.read_exact(&mut buffer).unwrap();
        println!("{}", String::from_utf8_lossy(&buffer));
        stream.write_all("hello client!\n".as_bytes()).unwrap();

        // create 1M buffer
        let buffer = vec![0; 1024 * 1024];
        for _i in 0..1024 {
            stream.write_all(buffer.as_slice()).unwrap();
        }
    }
}
