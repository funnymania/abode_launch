mod server;
mod uuid;
#[cfg(feature = "email")]
mod email;
#[cfg(feature = "email")]
mod personal_info;

use server::Server;
use std::env;

const PORT_DEV: u64 = 7565;
const PORT_PROD: u64 = 5657;

fn main() {
    println!("Hello, world!");

    // 'd' signifies dev, and we use a different port. 
    let args: Vec<String> = env::args().collect(); 
    if args.len() > 1 {
        println!("{}", args[1]);
        match args[1].as_str() {
           "d" => {
                Server::run(PORT_DEV);
           }
           _ => {
                Server::run(PORT_PROD);
           }
        }
    } else {
        Server::run(PORT_PROD);
    }
}
