mod client;
mod server;

use std::thread;
use std::time::Instant;

fn main() {
    let socket = "192.168.2.8:8080";
    // Start the server in a separate thread
    thread::spawn(|| {
        server::server::start_server(socket).unwrap();
    });

    // Give the server some time to start
    thread::sleep(std::time::Duration::from_secs(1));

    // Start timing
    let start = Instant::now();

    let calculator = client::client_proxy::Calculator::new(socket);
    println!("{}", calculator.add(1, 2));
    println!("{}", calculator.divide(10, 5));
    println!("{}", calculator.multiply(8, 3));
    println!("{}", calculator.subtract(16, 11));

    // Measure elapsed time
    let duration = start.elapsed();
    println!("Time taken to send message: {:?}", duration);
}