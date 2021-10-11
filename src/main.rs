mod server;
mod uuid;

use server::Server;

fn main() {
    println!("Hello, world!");
    //let serve = Server::new("funnymania");
    Server::run(5657);
}
