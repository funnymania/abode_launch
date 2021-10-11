mod server;
mod uuid;

use server::Server;
use std::env;

const PORT_DEV: u64 = 7565;
const PORT_PROD: u64 = 5657;

fn main() {
    println!("Hello, world!");

    // 'd' signifies dev, and we use a different port. 
    let args: Vec<String> = env::args().collect(); 
    if args.len() > 1 {
        match args[1].as_str() {
           "d" => {
                Server::run(7565);
           }
           _ => {
                Server::run(5657);
           }
        }
    } else {
        Server::run(5657);
    }
}
