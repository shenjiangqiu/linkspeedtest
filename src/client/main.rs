use std::{env::args, io::Read, net::TcpStream, time::Instant};
fn main() {
    let addr = args()
        .nth(1)
        .unwrap_or_else(|| "thesjq.com:2233".to_string());
    let size_m = args().nth(2).unwrap_or_else(|| "10".to_string());
    let size_m = size_m.parse::<usize>().unwrap();
    println!("Connecting to {}", addr);
    println!("Receiving {}MB", size_m);

    let mut stream = TcpStream::connect(addr.clone()).unwrap();
    println!(
        "Connected to {}, local address: {}",
        addr,
        stream.local_addr().unwrap()
    );
    let now = Instant::now();
    let mut buffer = vec![0; 1024 * 1024];

    for i in 0..size_m {
        stream.read_exact(&mut buffer).unwrap();

        println!("read {}MB", i + 1);
        println!(
            "average speed: {}MB/s",
            (i + 1) as f64 / now.elapsed().as_secs_f64()
        );
    }
    println!("total time: {} s", now.elapsed().as_secs_f64());
    println!(
        "speed: {} MB/s",
        size_m as f64 / now.elapsed().as_secs_f64()
    );
}
