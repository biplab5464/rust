use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");

    // Send data to the server
    let message = "Hello from client!";
    stream.write(message.as_bytes()).unwrap();

    // Receive and print the response from the server
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    println!("Server response: {}", response);
}
