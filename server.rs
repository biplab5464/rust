use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    // Read data from the client
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Process the received data (in this case, just print it)
            println!("Received: {}", String::from_utf8_lossy(&buffer));

            // Respond to the client
            let response = "Hello from server!";
            stream.write(response.as_bytes()).unwrap();
        }
        Err(e) => {
            eprintln!("Error reading from client: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    println!("Server listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread or use async/await for concurrent handling of multiple clients
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
