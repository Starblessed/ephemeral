use clap::Parser;
use log::info;

mod server;
pub mod session;
pub mod session_manager;
pub mod utils;

#[derive(Parser)]
struct Args {
    /// Host IP Address
    #[arg(long, env = "IP")]
    ip: String,

    /// Server Application Port
    #[arg(long, env = "PORT")]
    port: Option<u16>,
}
#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    let address: server::Address = {
        let port = match args.port {
            Some(port) => {
                if port != server::STANDARD_PORT {
                    info!("Custom port provided: {}", port);
                }
                port
            }
            None => {
                info!(
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
