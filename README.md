# CLI Chat Application

A simple CLI-based chat application built in Rust, featuring a server that handles multiple client connections and clients that can send and receive messages.

## Project Structure

```
src/
├── server_main.rs   # Entry point for the server
├── server.rs        # Core server logic
├── client_main.rs   # Entry point for the client
├── client.rs        # Core client logic
```

## Features

- **Server**: Handles multiple client connections and broadcasts messages to all connected clients.
- **Client**: Connects to the server, sends messages, and receives messages in real-time.

## How to Run

### Start the Server
Run the server using the following command:
```bash
cargo run --bin cli_chat_server
```

### Start a Client
In a new terminal window, start a client using:
```bash
cargo run --bin cli_chat_client
```

### Connect Multiple Clients
Repeat the client command in multiple terminal windows to connect more clients to the server.

## Commands (Client)
- **Send a message**: Type your message and press Enter to send it to all connected clients.
- **Quit**: Type `/quit` to disconnect from the server.

## Requirements

- Rust (latest stable version)
- Cargo (Rust package manager)

## License

This project is licensed under the MIT License.

