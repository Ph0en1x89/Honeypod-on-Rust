use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:22").unwrap();

    println!("SSH honeypot listening on port 22...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Connection succeeded
                println!("New SSH connection: {}", stream.peer_addr().unwrap());

                // Read incoming data from the client
                let mut buf = [0; 1024];
                let _ = stream.read(&mut buf);

                // Log the incoming data
                println!("Incoming data:\n{}", String::from_utf8_lossy(&buf));

                // Send a fake response to the client
                let response = b"SSH-2.0-OpenSSH_7.4p1 Debian-10+deb9u7\r\n";
                let _ = stream.write(response);
            }
            Err(e) => {
                // Connection failed
                println!("Error accepting SSH connection: {}", e);
            }
        }
    }
}
