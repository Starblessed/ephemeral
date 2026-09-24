use clap::Parser;

mod session;

#[derive(Parser)]
struct Args {
    /// Host IP Address
    ip: String,

    /// Server Application Port
    port: i32,
}

fn main() {

    let args = Args::parse();

    let port_string: String = args.port.to_string();

    println!("Provided address: {0}:{1}", args.ip, port_string);

    // TODO: add server run logic

}
