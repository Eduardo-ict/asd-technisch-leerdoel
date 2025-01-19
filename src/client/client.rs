use std::io::{Write, self, Read};
use std::net::TcpStream;

pub fn send_message(address: &str, message: &str) -> io::Result<String> {
    let mut stream = TcpStream::connect(address)?; // Maak verbinding met de server
    stream.write_all(message.as_bytes())?;         // Verstuur het bericht

    // Buffer om het antwoord van de server te ontvangen
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer)?;    // Lees het antwoord van de server

    // Print het antwoord van de server
    Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
}