use std::{
    env::args,
    io::{Read, Write},
    net::TcpStream,
    time::Instant,
};
fn main() {
    let addr = args()
        .nth(1)
        .unwrap_or_else(|| "localhost:2233".to_string());

    let mut stream = TcpStream::connect(addr.clone()).unwrap();
    println!(
        "Connected to {}, local address: {}",
        addr,
        stream.local_addr().unwrap()
    );
    stream.write_all("hello server!\n".as_bytes()).unwrap();
    let mut buffer = [0; 13];
    stream.read_exact(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));

    // create 1M buffer
    let now = Instant::now();
    let mut buffer = vec![0; 1024 * 1024];
    for _ in 0..1024 {
        stream.read_exact(buffer.as_mut_slice()).unwrap();
    }
    println!("total time: {}", now.elapsed().as_millis());
    println!("speed: {} MB/s", 1024.0 / now.elapsed().as_secs_f64());
}
