use std::env::args;
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let addr = args().nth(1).unwrap_or_else(|| ":::0".to_string());
    let size_m = args().nth(2).unwrap_or_else(|| "1024".to_string());
    let size_m = size_m.parse::<usize>().unwrap();
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
        for i in 0..size_m {
            stream.write_all(buffer.as_slice()).unwrap();
            println!("wrote {}M", i + 1);

        }
        stream.flush().unwrap();
        //close stream
        stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}
