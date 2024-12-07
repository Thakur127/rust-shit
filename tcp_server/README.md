# TCP Echo Server in Rust

This is a simple TCP server implemented in Rust. The server listens for incoming client connections, reads data from clients, and echoes the data back to the client. Additionally, it sends a `$` prompt to the client before reading input.

## Features

- **TCP server** that listens on `127.0.0.1:8080`.
- **Time-to-Live (TTL)** configuration for the socket to control packet lifetime.
- **Echo functionality**: The server reads client input and sends the same data back.
- **Prompts clients**: The server sends a `$` prompt to the client before reading input.
- **Graceful disconnection**: The server detects when a client disconnects.

## Dependencies

This project uses Rust's standard library, so there are no external dependencies.

## How to Build and Run

### 1. Clone the Repository

```bash
git clone https://github.com/Thakur127/rust-shit.git
cd rust-shit/tcp_server
```

### 2. Build the Project

Use `cargo` to build the project:

```bash
cargo build --release
```

### 3. Run the Server

After building the project, run the server with the following command:

```bash
cargo run
```

The server will start listening on `127.0.0.1:8080`, and you will see a message like:

```
listening on 127.0.0.1:8080
TTL: 100
```

### 4. Testing the Server

To test the server, you can use any TCP client, such as `telnet` or `nc` (Netcat).

#### Using `telnet`:

```bash
telnet 127.0.0.1 8080
```

Once connected, the server will send the `$` prompt. You can type anything, and the server will echo back the input:

```
$ Hello, server!
Hello, server!
$ How are you?
How are you?
```

#### Using `nc` (Netcat):

```bash
echo "Hello, server!" | nc 127.0.0.1 8080
```

This will send a message to the server and receive an echo response.

### 5. Stopping the Server

You can stop the server by pressing `Ctrl+C` in the terminal.

## Code Walkthrough

### `main.rs`

- **TcpListener::bind("127.0.0.1:8080")**: Binds the server to the specified IP and port (`127.0.0.1:8080`).
- **set_ttl(100)**: Configures the TTL (Time-To-Live) for the socket. This controls how long packets will live before being discarded.
- **listener.incoming()**: The server listens for incoming connections in a loop. For each connection, it passes the `TcpStream` to the `handle_connection` function.
- **handle_connection**: This function handles the reading and writing of data from and to the connected client.
  - **write_all(b"$ ")**: Sends a prompt to the client.
  - **read(&mut buffer)**: Reads data sent by the client.
  - **write_all(&buffer[..bytes_read])**: Echoes the data back to the client.
  - **flush()**: Ensures that all buffered data is written to the stream immediately.

## Error Handling

The code uses Rust's `Result` type for error handling, returning `Box<dyn std::error::Error>` for any errors that occur during execution. This provides a flexible way of handling different types of errors, such as network errors or I/O issues.
