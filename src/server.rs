extern crate postgres;
use postgres::{Client, NoTls};
#[cfg(feature = "email")]
use crate::email::Email;

use uuid::Uuid;
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

        // Setup email information
        #[cfg(feature = "email")]
        let emailer = Email::new("Abode", "mcclureDmichael", "funnymania.lol");

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
                    println!("Path: {}", str_req.1);
                    match str_req.1.as_str() {
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
                            let mut content = (String::new(), String::new());
                            match Server::extract_body(&req) {
                                Ok(body) => {
                                    match Server::validate_email(&body) {
                                        Ok(email) => {
                                            match Server::add_subscriber(&mut client, email) {
                                                Ok(res) => {
                                                    content.0 = String::from("Success");
                                                    content.1 = res;

                                                    #[cfg(feature = "email")]
                                                    emailer.send_to(email);
                                                }
                                                Err(e) => {
                                                    match e.as_str() {
                                                        "23505" => {
                                                            content.0 = String::from("Dupe");
                                                            content.1 = String::from("Email is already present! Thank you!");
                                                        },
                                                        _ => {
                                                        content.0 = String::from("Other");
                                                        content.1 = e;
                                                        }
                                                    }
                                                }
                                            };
                                        }
                                        Err(msg) => {
                                            content = msg;
                                        }
                                    }
                                }
                                Err(msg) => {
                                    println!("{}", msg);
                                    continue;
                                },
                            }

                            let content = format!("{{\n\"code\": \"{}\",\n\"msg\": \"{}\"\n}}", content.0, content.1);
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
                            // ApiString struct
                            let api_str = Server::is_valid_api(&str_req.1);
                            match api_str {
                                Some(api_call) => {
                                    match api_str.version {
                                        "v1" => {
                                            // get user by ID
                                            match api_str.value[0] {
                                                "user" => {
                                                    match str_req.0 {
                                                        "GET" => {
                                                            let user = Server::to_json(Server::get_user(&api_str.value[1]));
                                                            
                                                            //TODO Send response
                                                            let content = format!("{{\n\"user\": {{\n \"name\": \"{}\"\n}\n}}", user.0);
                                                            response = format!(
                                                                "HTTP/1.1 200 OK\r\n\
                                                                Content-Type: text/html\r\n\
                                                                Content-Length: {}\r\n\r\n{}",
                                                                content.len(),
                                                                content
                                                            );

                                                            stream.write(response.as_bytes()).unwrap();
                                                        }
                                                        "POST" => {
                                                            match Server::extract_body(&req) {
                                                                Ok(body) => {
                                                                    let result = Server::update_user(&body);
                                                                    let mut content = String::new();
                                                                    match result {
                                                                        Ok(user) => {
                                                                            content = format!("{{\n\"user\": {{\n \"name\": \"{}\"\n}\n}}", user.0);
                                                                        }
                                                                        Err(msg) => {
                                                                            content = format!("{{\n\"error\": \"{}\"}}", msg);
                                                                        }
                                                                    }
                                                                    response = format!(
                                                                        "HTTP/1.1 200 OK\r\n\
                                                                        Content-Type: text/html\r\n\
                                                                        Content-Length: {}\r\n\r\n{}",
                                                                        content.len(),
                                                                        content
                                                                    );

                                                                    stream.write(response.as_bytes()).unwrap();
                                                                }
                                                                Err(msg) => {}
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                        }
                                        _ => {}
                                    }
                                },
                                None => {
                            match Server::get_extension(&str_req.1) {
                                Ok(ext) => {
                                    let mut content = Vec::new();
                                    match ext {
                                        //TODO: Consider stripping evil things like '../..' from
                                        // the requested resource.
                                        "svg" => {
                                            match Server::get_file(format!("/rsrcs/{}",  str_req.1).as_str()) {
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

    pub fn whats_reqd(req: String) -> (String, String) {
        let mut lines: Vec<&str> = req.lines().collect();
        let first_line: Vec<&str> = lines[0].split(" ").collect();
        match first_line[0] {
            "GET" => {
                (String::from("GET"), String::from(first_line[1]))
            }
            "POST" => {
                (String::from("POST"), String::from(first_line[1]))
            }
            "PUT" => {
                (String::from("PUT"), String::from(first_line[1]))
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

    pub fn add_subscriber<'a>(client: &mut postgres::Client, email: &str) -> Result<String, String> {
        //generate unique ID
        let uuid = &Uuid::new_v4();

        let res = client.query("INSERT INTO subscriber VALUES($1, $2)", &[&uuid, &email]);
        match res {
            Ok(rows) => {
                Ok(String::from("Success"))
            }
            Err(e) => {
                Err(format!("{}", e.code().unwrap().code()))
            }
        }
    }

    /// Get all data after a double newline (beginning of HTTP Body)
    pub fn extract_body(req: &[u8]) -> Result<String, String> {
        if req.len() <= 3 {
            return Err("Empty Result".to_string());
        }

        let mut body_found = false;
        let mut body: Vec<u8> = Vec::new();
        for i in 3..req.len() {
            if !body_found {
                if req[i] == 10 && req[i - 1] == 13 && req[i-2] == 10 && req[i - 1] == 13   {
                    body_found = true;
                }
            } else if req[i] != 0 {
                body.push(req[i]); 
            }
        }

        Ok(String::from_utf8(body).unwrap())
    }

    pub fn validate_email(email: &str) -> Result<&str, (String, String)> {
        if email.len() > 255 {
            return Err((String::from("Email Format"), String::from("Email must be shorter than 256 characters")));
        }

        let email_parts = Server::email_address_parts(email);
        println!("email: {} {} {} {}", email, email_parts.0, email_parts.1, email_parts.2);
        if email_parts.0 == "" || email_parts.1 == "" || email_parts.2 == "" {
            return Err((String::from("Email Format"), String::from("Email must be in the proper format: you@example.com")));
        } 
        
        Ok(email)
    }

    pub fn email_address_parts(email: &str) -> (String, String, String) {
        let mut user = String::new();
        let mut host = String::new();
        let mut ext = String::new();

        // split at char '@'
        let mut foundAt = false;
        let mut foundHost = false;
        for ch in email.chars() {
            if foundAt {
                if foundHost {
                    ext += &ch.to_string()
                } else {
                    if ch == '.' {
                        foundHost = true;
                    } else {
                        host += &ch.to_string();
                    }
                }
            } else {
                if ch == '@' {
                    foundAt = true;
                } else {
                    user += &ch.to_string();
                }
            }
        }

        (user, host, ext)
    }

    pub fn update_user(json_user: &str) -> Result<(), String> {
        //TODO Convert JSON arg to searchable fields

        //TODO Update record.
    }
}
