use clap::Parser;

mod server;
pub mod session;
pub mod session_manager;
pub mod utils;

#[derive(Parser)]
struct Args {
    /// Host IP Address
    ip: String,

    /// Server Application Port
    port: Option<u16>,
}
#[tokio::main]
async fn main() {
    let args = Args::parse();

    let address: server::Address = {
        let port = match args.port {
            Some(port) => {
                if port != server::STANDARD_PORT {
                    println!("Custom port provided: {}", port);
                }
                port
            }
            None => {
                println!(
                    "No port provided, proceeding with standard: {}",
                    server::STANDARD_PORT
                );
                server::STANDARD_PORT
            }
        };

        server::Address::from_tuple((args.ip, port))
    };

    let server = server::Server::from_address(address);

    _ = server.run().await; // . . .
}
