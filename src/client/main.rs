use std::{
    env::args,
    io::{Read, Write},
    net::{TcpStream, Shutdown},
    time::Instant,
};
fn main() {
    let addr = args()
        .nth(1)
        .unwrap_or_else(|| "localhost:2233".to_string());
    let size_m = args().nth(2).unwrap_or_else(|| "1024".to_string());
    let size_m = size_m.parse::<usize>().unwrap();

    let mut stream = TcpStream::connect(addr.clone()).unwrap();
    println!(
        "Connected to {}, local address: {}",
        addr,
        stream.local_addr().unwrap()
    );
    stream.write_all("hello server!\n".as_bytes()).unwrap();
    let mut buffer = [0; 14];
    stream.read_exact(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));

    // create 1M buffer
    let now = Instant::now();
    let mut buffer = vec![0; 1024 * 1024];
    for i in 0..size_m {
        stream.read_exact(buffer.as_mut_slice()).unwrap();
        println!("read {}M", i + 1);
        println!(
            "average speed: {}M/s",
            (i + 1) as f64 / now.elapsed().as_secs_f64()
        );
    }
    println!("total time: {}s", now.elapsed().as_secs_f64());
    println!(
        "speed: {} MB/s",
        size_m as f64 / now.elapsed().as_secs_f64()
    );
    stream.shutdown(Shutdown::Both).unwrap();
}
