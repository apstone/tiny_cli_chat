use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind to address");

    let clients = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected: {}", stream.peer_addr().unwrap());

                let clients = Arc::clone(&clients);

                // Spawn a new thread to handle the client connection
                std::thread::spawn(move || {
                    handle_client(stream, clients);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    println!("Server is running on 127.0.0.1:8080");
}

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    // Lock the clients list and add the new client
    let mut clients_lock = clients.lock().unwrap();
    clients_lock.push(stream.try_clone().expect("Failed to clone stream"));

    println!("Client connected. Total clients: {}", clients_lock.len());
}
