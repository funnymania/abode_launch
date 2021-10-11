extern crate postgres;
use postgres::{Client, NoTls};

use crate::uuid;
use std::net::TcpListener;
use std::io::Read;
use std::io::Write;
use std::thread;
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

        // Start postgres client
        let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

        // open Log file
        let mut log_file = Server::tail_file().unwrap();

        for stream in listener.incoming() {
            println!("Got one!");
            
            let mut stream = stream.unwrap();
            //Browser extensions CRAM a lot of extra data into the cookie header. 
            //It is not an Error for this buffer to be too small, so we won't catch it
            let mut req = [0; 2048];

            // Split to different actions
            let mut response = String::new();
            match stream.read(&mut req) {
                Err(msg) => println!("{}", msg),
                Ok(bytes_read) => {
                    // If the requests contain a lot of data, we will log it, because we are curious how
                    // people are requesting, and then skip it.
                    if bytes_read > 1024 {
                        Server::add_to_file(&log_file, &req);
                        continue;
                    }

                    println!("Byte of reqs: {}", bytes_read);
                    let str_req = Server::whats_reqd(String::from_utf8_lossy(&req).to_string());
                    println!("Path: {}", str_req);
                    match str_req.as_str() {
                        "/" | "/wuh???" => {
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

                            match stream.write_all(response.as_bytes()) {
                                Err(msg) => println!("Error: {}\n{}", msg, String::from_utf8_lossy(&req)),
                                Ok(num) => (),
                            }
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
                        "/global.css" => {
                            let content = match Server::get_page("/rsrcs/global.css") {
                                Ok(html) => html,
                                Err(e) => format!("<html><body>Webpage was not formatted correctly, please @funnymania_ in case they are sleeping (Zzzz):<br><a href=\"https://twitter.com/funnymania_\">https://twitter.com/funnymania_</a><br><br>Error: {}</body></html>", e)
                            };
                            response = format!(
                                "HTTP/1.1 200 OK\r\n\
                                Content-Type: text/css\r\n\
                                Content-Length: {}\r\n\r\n{}",
                                content.len(),
                                content
                            );

                            match stream.write(response.as_bytes()) {
                                Err(msg) => println!("Error: {}\n{}", msg, String::from_utf8_lossy(&req)),
                                _ => ()
                            }
                        }
                        "/subscribe" => {
                            let content = match Server::get_page("/views/subscribe.html") {
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

                            match stream.write(response.as_bytes()) {
                                Err(msg) => println!("Error: {}\n{}", msg, String::from_utf8_lossy(&req)),
                                _ => ()
                            }
                        }

                        "/why???" => {
                            let content = match Server::get_page("/views/why???.html") {
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

                            match stream.write(response.as_bytes()) {
                                Err(msg) => println!("Error: {}\n{}", msg, String::from_utf8_lossy(&req)),
                                _ => ()
                            }
                        }
                        "/pre-release" => {
                            let content = match Server::get_page("/views/pre-release.html") {
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

                            match stream.write(response.as_bytes()) {
                                Err(msg) => println!("Error: {}\n{}", msg, String::from_utf8_lossy(&req)),
                                _ => ()
                            }
                        }
                        "/installs" => {
                            let mut content = String::new();
                            match Server::get_installs(&mut client) {
                                Ok(num) => content = num.to_string(),
                                Err(e) => content = "0".to_string(),
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
                        "/subscriber" => {
                            let mut content = String::new();
                            println!("{:?}", req);
                            // match Server::add_subscriber(&mut client) {
                            //     Ok(res) => content = res,
                            //     Err(e) => content = "Other".to_string(),
                            // };

                            response = format!(
                                "HTTP/1.1 200 OK\r\n\
                                Content-Type: text/html\r\n\
                                Content-Length: {}\r\n\r\n{}",
                                content.len(),
                                content
                            );

                            stream.write(response.as_bytes()).unwrap();
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
                            match Server::get_extension(&str_req) {
                                Ok(ext) => {
                                    let mut content = Vec::new();
                                    match ext {
                                        //TODO: Consider stripping evil things like '../..' from
                                        // the requested resource.
                                        "svg" => {
                                            match Server::get_file(format!("/rsrcs/{}",  str_req).as_str()) {
                                                Ok(mut svg) => {
                                                    svg.read_to_end(&mut content);
                                                }
                                                Err(e) => {
                                                    format!("<html><body>Webpage was not formatted correctly, please @funnymania_ in case they are sleeping (Zzzz):<br><a href=\"https://twitter.com/funnymania_\">https://twitter.com/funnymania_</a><br><br>Error: {}</body></html>", e);
                                                }
                                            };
                                            println!("SVG bytes read: {}", content.len());
                                            response = format!(
                                                "HTTP/1.1 200 OK\r\n\
                                                Content-Type: image/svg+xml\r\n\
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
                                        _ => (),
                                    }
                                },
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
                }
            }
            // print!("\n{}", String::from_utf8_lossy(&req[..]));
            
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
        let mut correct_path = format!("{}", env!("CARGO_MANIFEST_DIR"));
        correct_path += file_path;
        let mut file = fs::OpenOptions::new().read(true).open(correct_path); 

        file
    }

    pub fn whats_reqd(req: String) -> String {
        let mut lines: Vec<&str> = req.lines().collect();
        let first_line: Vec<&str> = lines[0].split(" ").collect();
        match first_line[0] {
            "GET" => {
                String::from(first_line[1])
            }
            "POST" => {
                String::from(first_line[1])
            }
            _ => {
                if first_line.len() < 2 {
                    String::from("")
                } else {
                    String::from(first_line[1])
                }
            }
        }
    }

    pub fn get_installs(client: &mut postgres::Client) -> Result<i64, String> {
        let res = client.query("SELECT value FROM installs", &[]);
        match res {
            Ok(rows) => {
                let installs: i64 = rows[0].get("value");
                Ok(installs)
            }
            Err(e) => Err(format!("{}", e)),
        }
    }

    /// Return a result if success, etc. Intent is to be aware of whenever files are 
    pub fn email_dev() {}

    pub fn get_extension(path: &str) -> Result<&str, String> {
        let mut left: usize = 0;
        let right = path.len();
       for p in path.char_indices().rev() {
            if p.1 == '.' {
                left = p.0 + 1;
                break;
            }
       }

       if left == 0 {
           Err("".to_string()) 
       } else {
           println!("{}", &path[left..right]);
           Ok(&path[left..right])
       }
    }

    pub fn tail_file() -> io::Result<fs::File> {
        let mut file_path = format!("{}", env!("CARGO_MANIFEST_DIR"));
        file_path += "/logs/tail.txt";
        let mut file = fs::OpenOptions::new().create(true).read(true).write(true).open(file_path);

        file
    }

    pub fn add_to_file(mut file: &fs::File, req: &[u8]) {
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        if contents.len() + req.len() < 4096 {
           file.write(req);
        }
    }

    pub fn add_subscriber<'a>(client: &mut postgres::Client, email: String) -> Result<String, String> {
        //generate unique ID
        let uuid = &uuid::create()[..]; 

        let res = client.query("INSERT INTO subscribers VALUES($1, $2)", &[&uuid, &email]);
        match res {
            Ok(rows) => {
                Ok(String::from("Success"))
            }
            Err(e) => Err(format!("{}", e)),
        }
        
    }
}
