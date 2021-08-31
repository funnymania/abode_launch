use std::net::TcpStream;
use std::net::TcpListener;
use std::io::Read;
use std::io::Write;
use std::io;
use std::fs;

pub struct Server {
    name: String,
}

impl Server {
    pub fn new(name: &str) -> Server {
        Server {
            name: name.to_string()
        }
    }

    pub fn run(port: u64) {
        let mut address = String::from("0.0.0.0:");
        address += &port.to_string();
        let listener = TcpListener::bind(&address).unwrap();

        for stream in listener.incoming() {
            println!("Got one!");
            
            let mut stream = stream.unwrap();
            let mut req = [0; 512];

            // Split to different actions
            let mut response = String::new();
            match stream.read(&mut req) {
                Err(msg) => println!("{}", msg),
                Ok(bytes_read) => {
                    println!("{}", bytes_read);
                    match &Server::whats_reqd(req) {
                        "/" => {
                            let content = match Server::get_page("/views/landing.html") {
                                Ok(html) => html,
                                Err(e) => format!("<html><body>Webpage was not formatted correctly, please @funnymania_ in case they are sleeping (Zzzz):<br><a href=\"https://twitter.com/funnymania_\">https://twitter.com/funnymania_</a><br><br>Error: {}</body></html>", e)
                            };
                            response = format!(
                                "HTTP/1.1 200 OK\r\n
                                Content-Length: {}\r\n\r\n{}",
                                content.len(),
                                content
                            );
                        }
                        "abodeCLI" => {
                            match Server::get_file("~/abode/target/release/abode.zip") {
                                Ok(file) => {
                                    let mut blob = [u8; 256];   
                                    file.read(&mut blob);
                                    response = format!(
                                        "HTTP/1.1 200 OK\r\n
                                        Content-Disposition: attachment; filename=\"abode.zip\"\r\n
                                        Content-Length: {}\r\n\r\n{}", 
                                        blob.len(),
                                        blob
                                    );
                                }
                                Err(e) => format!("<html><body>Webpage was not formatted correctly, please @funnymania_ in case they are sleeping (Zzzz):<br><a href=\"https://twitter.com/funnymania_\">https://twitter.com/funnymania_</a><br><br>Error: {}</body></html>", e)
                            }
                        }
                        - => {
                            content = match Server::get_page("/views/whats-that.html") {
                                Ok(html) => html,
                                Err(e) => format!("<html><body>Webpage was not formatted correctly, please @funnymania_ in case they are sleeping (Zzzz):<br><a href=\"https://twitter.com/funnymania_\">https://twitter.com/funnymania_</a><br><br>Error: {}</body></html>", e)
                            };

                            response = format!(
                                "HTTP/1.1 200 OK\r\n
                                Content-Length: {}\r\n\r\n{}",
                                content.len(),
                                content
                            );
                        }
                    }
                }
            }
            print!("\n{}", String::from_utf8_lossy(&req[..]));
            
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }

    pub fn get_page(page: &str) -> io::Result<String> {
        let mut file_path = format!("{}", env!("CARGO_MANIFEST_DIR"));
        file_path += page;
        let mut file = fs::OpenOptions::new().read(true).open(file_path.as_str())?;
        
        let mut web_page: String = String::new();

        file.read_to_string(&mut web_page)?;

        Ok(web_page)
    }

    pub fn get_file(file_path: String) -> io::Result<File> {
       let file = fs::OpenOptions(file_path).read()?; 

       file
    }

    /// return parsed get resource
    pub fn whats_reqd(req: String) -> io::Result<String> {

    }
}
