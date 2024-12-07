use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // bind to socket
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("listening on 127.0.0.1:8080");

    // set ttl(time to live) on every packet sent from the server
    listener.set_ttl(100).expect("error setting TTL");
    println!("TTL: {}", listener.ttl().unwrap()); // .ttl() returns the current TTL

    // listen for connections
    // match listener.accept() {
    //     Ok((_socket, addr)) => println!("new client: {addr:?}"),
    //     Err(e) => println!("couldn't get client: {e:?}"),
    // }

    // equivalent of .accept() in a loop
    for stream in listener.incoming() {
        handle_connection(stream?)?;
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];

    // read from stream
    loop {
        stream.write_all(b"$ ")?;

        // reads client input
        let bytes_read = stream.read(&mut buffer)?;
        // println!("bytes read: {bytes_read}");

        if bytes_read == 0 {
            println!("client disconnected");
            return Ok(());
        }

        println!("$ {}", String::from_utf8_lossy(&buffer[..bytes_read]));

        // write to stream
        stream.write_all(&buffer[..bytes_read])?;

        // flush stream to ensure buffered data reaches the desitnation
        stream.flush()?;
    }
}
