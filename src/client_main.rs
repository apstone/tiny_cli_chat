mod client;

fn main() {
    let server_address = "127.0.0.1:8080";
    println!("Starting the chart client...");

    client::start_client(server_address);
}
