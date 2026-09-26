use crate::session_manager::SessionManager;
use datetime::Instant;

use tokio::io::Result;
use tokio::net::TcpListener;

use std::fmt;

pub const STANDARD_PORT: u16 = 8411;

pub struct Address {
    ip: String,
    port: u16,
}

impl Address {
    pub fn from_tuple((ip, port): (String, u16)) -> Address {
        Address { ip, port }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

pub struct Server {
    started: Instant,
    manager: SessionManager,
    address: Address,
}

impl Server {
    pub fn from_address(address: Address) -> Server {
        Server {
            started: Instant::now(),
            manager: SessionManager::new(),
            address: address,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.address.to_string()).await?;
        println!("Server started at: {:?}", self.started);
        println!("Ephemeral Server listening on {}", self.address);

        // TODO: Accept connections and redirect them to the session manager

        loop {
            let (socket, client_addr) = listener.accept().await?;
            println!("New connection established with: {}", client_addr);

            self.manager.add_socket(socket).await;
        }
    }
}
