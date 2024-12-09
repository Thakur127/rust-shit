# TCP Echo Server in Rust

This is a simple TCP server implemented in Rust that listens for incoming client connections. It accepts Bash commands from clients, executes them, and sends the output back to the client.

## Features

- **TCP server**: Listens on `127.0.0.1:8080` for incoming client connections.
- **Command execution**: Clients can send Bash commands to the server, and the server will execute them and return the result.
- **Graceful disconnection**: The server can detect when a client disconnects and cleanly shuts down the connection.

## Dependencies

This project uses only Rust's standard library, so there are no external dependencies.

## How to Build and Run

### 1. Clone the Repository

Clone the repository to your local machine:

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

After building the project, you can run the server with the following command:

```bash
cargo run
```

The server will start listening on `127.0.0.1:8080`, and you will see a message like:

```
listening on 127.0.0.1:8080
TTL: 100
```

### 4. Testing the Server

You can test the server using any TCP client such as `telnet` or `nc` (Netcat).

#### Using `telnet`:

```bash
telnet 127.0.0.1 8080
```

Once connected, you can type any valid Bash command. The server will execute the command and send the output back to the client:

```
$ ls
file1.txt
file2.txt
$ pwd
/home/user
```

#### Using `nc` (Netcat):

You can use `echo` with `nc` (Netcat) to send a command to the server:

```bash
echo "ls" | nc 127.0.0.1 8080
```

This will send the `ls` command to the server, and the server will return the output, which might look like this:

```
file1.txt
file2.txt
```

### 5. Stopping the Server

You can stop the server by pressing `Ctrl+C` in the terminal.

## Code Walkthrough

### `main.rs`

- **`TcpListener::bind("127.0.0.1:8080")`**: This binds the server to the IP address `127.0.0.1` and port `8080`.
- **`set_ttl(100)`**: This configures the Time-To-Live (TTL) for the socket, determining how long packets will live before being discarded. This is used to control network traffic behavior.
- **`listener.incoming()`**: This listens for incoming client connections and processes each connection by calling the `handle_connection` function.
- **`handle_connection`**: This function manages communication with a single client. It:
  - **Reads commands**: It reads commands sent by the client.
  - **Executes commands**: It executes each command using the `run_bash_command` function.
  - **Sends results back**: The output of the command is sent back to the client.
  - **Handles disconnections**: If the client disconnects, the server logs the disconnection and cleans up.

### `run_bash_command`

The `run_bash_command` function is responsible for executing Bash commands sent by the client. It runs the command using Rust's `std::process::Command` API and returns the command's output or any error that occurred during execution.

---

## Error Handling

The server uses Rust's `Result` type for error handling. It will gracefully handle connection issues, command execution errors, and network problems. If a client disconnects, the server will terminate the connection cleanly without crashing.

---

## Conclusion

This TCP Echo Server allows clients to send Bash commands and receive the output. It demonstrates the power and flexibility of Rust for building networked applications and handling real-time command execution. The server is robust, easy to run, and can be easily extended to support more advanced functionality.
