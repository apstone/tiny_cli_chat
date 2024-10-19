use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind to address");

    let clients = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peer_addr = stream.peer_addr().unwrap();
                println!("New client connected: {}", peer_addr);

                let clients = Arc::clone(&clients);

                {
                    let mut clients_lock = clients.lock().unwrap();
                    clients_lock.push(stream.try_clone().expect("Failed to clone stream"));
                }

                // Spawn a new thread to handle the client connection
                std::thread::spawn(move || {
                    handle_client(stream, clients);
                    println!("Client {} disconnected.", peer_addr);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    println!("Server is running on 127.0.0.1:8080");
}

fn handle_client(stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let peer_addr = stream.peer_addr().unwrap();
    let reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));

    // Loop to continuously read messages from the client
    for line in reader.lines() {
        match line {
            Ok(message) => {
                let formatted_message = format!("{}: {}", peer_addr, message);

                // Print the message to the server console (for debugging)
                println!("{}", formatted_message);

                // Print the message to all connected clients
                broadcast_message(&formatted_message, &stream, clients.clone());
            }
            Err(e) => {
                eprintln!("Error reading from the client {}: {}", peer_addr, e);
                break;
            }
        }
    }

    // Remove the client from the list when they disconnect
    remove_client(&stream, clients);
}

fn broadcast_message(message: &str, sender: &TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let clients_lock = clients.lock().unwrap();

    // Loop through all connected clients and send the message
    for client in clients_lock.iter() {
        // Skip the sender
        if client.peer_addr().unwrap() == sender.peer_addr().unwrap() {
            continue;
        }

        let mut client = client;

        let message_with_newline = format!("{}\n", message);

        if let Err(e) = client.write_all(message_with_newline.as_bytes()) {
            eprintln!(
                "Failed to send message to {}: {}",
                client.peer_addr().unwrap(),
                e
            );
        }

        if let Err(e) = client.flush() {
            eprintln!(
                "Failed to flush message to {}: {}",
                client.peer_addr().unwrap(),
                e
            );
        }
    }
}

fn remove_client(stream: &TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut clients_lock = clients.lock().unwrap();
    let index = clients_lock
        .iter()
        .position(|client| client.peer_addr().unwrap() == stream.peer_addr().unwrap());

    if let Some(i) = index {
        clients_lock.remove(i);
        println!("Removed client: {}", stream.peer_addr().unwrap());
    }
}
