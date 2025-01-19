use std::io::{self, Read, Write};
use std::net::TcpListener;
use std::str;
use crate::server::server_proxy;

pub fn start_server(address: &str) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("Server is running on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 512];
                let bytes_read = stream.read(&mut buffer)?;
                let received = str::from_utf8(&buffer[..bytes_read]).unwrap();
                println!("Received: {}", received);

                // Verwerk het bericht en voer de functie uit
                let response = server_proxy::process_request(received);
                stream.write_all(response.as_bytes())?;
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}
