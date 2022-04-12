use std::env::args;
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let addr = args().nth(1).unwrap_or_else(|| ":::2233".to_string());

    let listener = TcpListener::bind(addr).unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    while let Ok((mut stream, addr)) = listener.accept() {
        println!("Accepted connection from {}", addr);
        // read the header of usize
        let mut buffer = [0; 8];
        stream.read_exact(&mut buffer).unwrap();
        let size_m = u64::from_be_bytes(buffer);
        println!("sending {}MB", size_m);
        if size_m == 0 {
            println!("Received stop signal");
            break;
        }
        // create 1M buffer
        let buffer = vec![0; 1024 * 1024];
        for i in 0..size_m {
            stream.write_all(&buffer).unwrap();
            println!("write {}MB", i + 1);
        }
        stream.flush().unwrap();
        //close stream
        // stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}
