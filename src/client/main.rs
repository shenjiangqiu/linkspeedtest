use std::{
    env::args,
    io::{Read, Write},
    net::TcpStream,
    time::Instant,
};
fn main() {
    let addr = args()
        .nth(1)
        .unwrap_or_else(|| "thesjq.com:2233".to_string());
    let size_m = args().nth(2).unwrap_or_else(|| "10".to_string());
    let size_m = size_m.parse::<u64>().unwrap();
    println!("Connecting to {}", addr);
    println!("Receiving {}MB", size_m);
    if size_m == 0 {
        println!("sending stop signal");
    }
    let mut stream = TcpStream::connect(addr.clone()).unwrap();
    println!(
        "Connected to {}, local address: {}",
        addr,
        stream.local_addr().unwrap()
    );
    // send header
    let size_m = size_m as u64;
    let size_m_ = size_m.to_be_bytes();
    stream.write_all(&size_m_).unwrap();
    stream.flush().unwrap();
    println!("Sent header");
    if size_m == 0 {
        println!("sent stop signal");
        return;
    }
    let now = Instant::now();

    let mut buffer = vec![0; 10 * 1024 * 1024];

    for i in 0..size_m {
        stream.read_exact(&mut buffer).unwrap();

        println!("read {} MB", (i + 1) * 10);
        println!(
            "average speed: {}MB/s",
            ((i + 1) * 10) as f64 / now.elapsed().as_secs_f64()
        );
    }
    println!("total time: {} s", now.elapsed().as_secs_f64());
    println!(
        "speed: {} MB/s",
        size_m as f64 / now.elapsed().as_secs_f64()
    );
}
