use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, Sender};
use std::thread;

pub fn start_client(server_address: &str) {
    // Connect to the server
    let stream = TcpStream::connect(server_address).expect("Could not connect to the server");
    println!("Connected to the server at {}", server_address);

    // Clone the stream to share between threads
    let write_stream = stream.try_clone().expect("Failed to clone stream");

    // Create a channel to communicate between the main thread and the listening thread
    let (tx, rx) = mpsc::channel::<String>();

    //Spawn a thread to listen for incoming messages from the server
    thread::spawn(move || {
        let reader = BufReader::new(stream);

        for line in reader.lines() {
            match line {
                Ok(message) => {
                    println!("Recieved: {}", message);
                }
                Err(e) => {
                    eprintln!("Error reading from the server: {}", e);
                    break;
                }
            }
        }
    });

    handle_user_input(write_stream, tx);
}

fn handle_user_input(mut stream: TcpStream, tx: Sender<String>) {
    println!("Type your messages below:");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let trimmed = input.trim();
        if trimmed == "/quit" {
            println!("Disconnecting...");
            break;
        }

        // Send the message to the server
        if let Err(e) = stream.write_all(trimmed.as_bytes()) {
            eprintln!("Failed to send the message: {}", e);
            break;
        }
    }
}
