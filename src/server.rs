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
                    match Server::whats_reqd(String::from_utf8_lossy(&req).to_string()).as_str() {
                        "/" => {
                            let content = match Server::get_page("/views/landing.html") {
                                Ok(html) => html,
                                Err(e) => format!("<html><body>Webpage was not formatted correctly, please @funnymania_ in case they are sleeping (Zzzz):<br><a href=\"https://twitter.com/funnymania_\">https://twitter.com/funnymania_</a><br><br>Error: {}</body></html>", e)
                            };
                            response = format!(
                                "HTTP/1.1 200 OK\r\n\
                                Content-Type: text/html\r\n\
                                Content-Length: {}\r\n\r\n{}",
                                content.len(),
                                content
                            );
                            stream.write(response.as_bytes()).unwrap();
                        }
                        "/favicon.ico" => {
                            let mut content = Vec::new();
                            match Server::get_file("/rsrcs/favicon.png") {
                                Ok(mut icon) => {
                                    icon.read_to_end(&mut content);
                                }
                                Err(e) => {
                                    format!("<html><body>Webpage was not formatted correctly, please @funnymania_ in case they are sleeping (Zzzz):<br><a href=\"https://twitter.com/funnymania_\">https://twitter.com/funnymania_</a><br><br>Error: {}</body></html>", e);
                                }
                            };
                            println!("Icon bytes read: {}", content.len());
                            response = format!(
                                "HTTP/1.1 200 OK\r\n\
                                Content-Type: image/png\r\n\
                                Content-Length: {}\r\n\r\n",
                                content.len(),
                            );
                            let mut byte_res: Vec<u8> = Vec::new();
                            for byte in response.as_bytes() {
                                byte_res.push(*byte);
                            }

                            for byte in content {
                                byte_res.push(byte);
                            }
                            
                            stream.write(&byte_res).unwrap();
                        }
                        // "abodeCLI" => {
                        //     match Server::get_file("~/abode/target/release/abode.zip") {
                        //         Ok(file) => {
                        //             let mut blob = [0; 256];   
                        //             file.read(&mut blob);
                        //             response = format!(
                        //                 "HTTP/1.1 200 OK\r\n
                        //                 Content-Disposition: attachment; filename=\"abode.zip\"\r\n
                        //                 Content-Length: {}\r\n\r\n{}", 
                        //                 blob.len(),
                        //                 blob
                        //             );
                        //         }
                        //         Err(e) => println!("<html><body>Webpage was not formatted correctly, please @funnymania_ in case they are sleeping (Zzzz):<br><a href=\"https://twitter.com/funnymania_\">https://twitter.com/funnymania_</a><br><br>Error: {}</body></html>", e)
                        //     }
                        // }
                        _ => {
                            let content = match Server::get_page("/views/whats-that.html") {
                                Ok(html) => html,
                                Err(e) => format!("<html><body>Webpage was not formatted correctly, please @funnymania_ in case they are sleeping (Zzzz):<br><a href=\"https://twitter.com/funnymania_\">https://twitter.com/funnymania_</a><br><br>Error: {}</body></html>", e)
                            };

                            response = format!(
                                "HTTP/1.1 200 OK\r\n\
                                Content-Length: {}\r\n\r\n{}",
                                content.len(),
                                content
                            );
                            stream.write(response.as_bytes()).unwrap();
                        }
                    }
                }
            }
            print!("\n{}", String::from_utf8_lossy(&req[..]));
            
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

    pub fn get_file(file_path: &str) -> io::Result<fs::File> {
       let mut file = fs::OpenOptions::new().read(true).open(file_path); 

       file
    }

    /// TODO: return parsed get resource
    pub fn whats_reqd(req: String) -> String {
        let mut lines: Vec<&str> = req.lines().collect();
        let first_line: Vec<&str> = lines[0].split(" ").collect();
        println!("{}", lines[0]);
        match first_line[0] {
            "GET" => {
                String::from(first_line[1])
            }
            "POST" => {
                String::from(first_line[1])
            }
            _ => String::from(first_line[1])
        }
    }
}
