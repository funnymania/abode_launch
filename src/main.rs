#[cfg(feature = "email")]
mod email;
#[cfg(feature = "email")]
mod personal_info;
mod server;
mod uuid;

use server::Server;
use std::env;

const PORT_DEV: u64 = 7565;
const PORT_DEV_HTTP: u64 = 7566;
const PORT_PROD: u64 = 5657;
const PORT_PROD_HTTP: u64 = 5658;

fn main() {
    println!("Hello, world!");

    // 'd' signifies dev, and we use a different port.
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "d" => {
                Server::run(PORT_DEV, PORT_DEV_HTTP);
            }
            _ => {
                Server::run(PORT_PROD, PORT_PROD_HTTP);
            }
        }
    } else {
        Server::run(PORT_PROD, PORT_PROD_HTTP);
    }
}
