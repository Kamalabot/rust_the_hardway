use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5855").unwrap();
    for stream in listener.incoming(){
        let unwrapstrm = stream.unwrap();
        println!("Connected... Get moviing");
        handle_frames(unwrapstrm);
    }
}

fn handle_frames(mut stream: TcpStream){
    let mut buf: [u8; 1024] = [0; 1024];
    // the read is enabled after prelude module is imported
    stream.read(&mut buf).unwrap();
    println!(
        "Request: {}", 
        String::from_utf8_lossy(&buf[..])
        // some methods have specific type of data, even within integers
    );
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
