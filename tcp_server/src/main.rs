use core::str;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;

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
        let mut pwd = run_bash_command("pwd")?;
        pwd = pwd.trim_end().to_string();
        pwd.push_str(" $ ");

        stream.write_all(pwd.as_bytes())?;

        // reads client input
        let bytes_read = stream.read(&mut buffer)?;
        // println!("bytes read: {bytes_read}");

        if bytes_read == 0 {
            println!("client disconnected");
            return Ok(());
        }

        let command = String::from_utf8_lossy(&buffer[..bytes_read]);

        let output = match run_bash_command(&command.trim()) {
            Ok(output) => output,
            Err(e) => e,
        };

        println!("$ {}", command);

        // write to stream
        stream.write_all(output.as_bytes())?;

        // flush stream to ensure buffered data reaches the desitnation
        stream.flush()?;
    }
}

fn run_bash_command(command: &str) -> Result<String, String> {
    let output = Command::new("bash")
        .arg("-c") // Tell Bash to execute the string following the -c
        .arg(command)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(format!("Error executing command: {}", e)),
    }
}
