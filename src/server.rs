extern crate postgres;
#[cfg(feature = "email")]
use crate::email::Email;
use postgres::{Client, NoTls};

use json::object;
#[cfg(feature = "https")]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslStream};
use uuid::Uuid;

use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ApiString {
    version: String,
    value: Vec<String>,
}

pub struct Server {
    name: String,
}

impl Server {
    pub fn new(name: &str) -> Server {
        Server {
            name: name.to_string(),
        }
    }

    pub fn handle_https<T: Read + Write>(
        mut stream: T,
        log_file: Arc<fs::File>,
        mut client: Arc<Mutex<postgres::Client>>,
    ) {
        // Setup email information
        #[cfg(feature = "email")]
        let emailer = Email::new("Abode", "mcclureDmichael", "funnymania.lol");

        //Browser extensions CRAM a lot of extra data into the cookie header.
        //It is not an Error for this buffer to be too small, so we won't catch it
        let mut req = [0; 2048];

        // Split to different actions
        let mut response = String::new();
        match stream.read(&mut req) {
            Err(msg) => println!("Stream read error {}", msg),
            Ok(bytes_read) => {
                println!("Byte of reqs: {}", bytes_read);

                // If the requests contain a lot of data, we will log it, because we are curious how
                // people are requesting, and then skip it.
                if bytes_read > 1024 {
                    Server::add_to_file(&log_file, &req);
                    return;
                }

                // 404 users if they send any non-utf8 data in request
                let mut check_req = String::new();
                match std::str::from_utf8(&req) {
                    Ok(checked_req) => check_req = checked_req.to_string(),
                    Err(e) => {
                        let content = match Server::get_page("/views/whats-that.html") {
                                Ok(html) => html,
                                Err(e) => format!("<html><body>Webpage was not formatted correctly, please @funnymania_ in case they are sleeping (Zzzz):<br><a href=\"https://twitter.com/funnymania_\">https://twitter.com/funnymania_</a><br><br>Error: {}</body></html>", e)
                            };

                        response = format!(
                            "HTTP/1.1 404 Not Found\r\n\
                            Content-Length: {}\r\n\r\n{}",
                            content.len(),
                            content
                        );

                        stream.write_all(response.as_bytes()).unwrap();
                        return;
                    }
                }

                let str_req = Server::whats_reqd(check_req);
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
                            Err(msg) => {
                                println!("Error: {}\n{}", msg, String::from_utf8_lossy(&req))
                            }
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

                        stream.write_all(&byte_res).unwrap();
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

                        match stream.write_all(response.as_bytes()) {
                            Err(msg) => {
                                println!("Error: {}\n{}", msg, String::from_utf8_lossy(&req))
                            }
                            _ => (),
                        }
                    }
                    "/login-page" => {
                        let content = match Server::get_page("/views/login.html") {
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
                            Err(msg) => {
                                println!("Error: {}\n{}", msg, String::from_utf8_lossy(&req))
                            }
                            _ => (),
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

                        match stream.write_all(response.as_bytes()) {
                            Err(msg) => {
                                println!("Error: {}\n{}", msg, String::from_utf8_lossy(&req))
                            }
                            _ => (),
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

                        match stream.write_all(response.as_bytes()) {
                            Err(msg) => {
                                println!("Error: {}\n{}", msg, String::from_utf8_lossy(&req))
                            }
                            _ => (),
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

                        match stream.write_all(response.as_bytes()) {
                            Err(msg) => {
                                println!("Error: {}\n{}", msg, String::from_utf8_lossy(&req))
                            }
                            _ => (),
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
                        stream.write_all(response.as_bytes()).unwrap();
                    }
                    "/subscriber" => {
                        let mut content = (String::new(), String::new());
                        match Server::extract_body(&req) {
                            Ok(body) => match Server::validate_email(&body) {
                                Ok(email) => {
                                    match Server::add_subscriber(&mut client, email) {
                                        Ok(res) => {
                                            content.0 = String::from("Success");
                                            content.1 = res;

                                            #[cfg(feature = "email")]
                                            emailer.send_to(email);
                                        }
                                        Err(e) => match e.as_str() {
                                            "23505" => {
                                                content.0 = String::from("Dupe");
                                                content.1 = String::from(
                                                    "Email is already present! Thank you!",
                                                );
                                            }
                                            _ => {
                                                content.0 = String::from("Other");
                                                content.1 = e;
                                            }
                                        },
                                    };
                                }
                                Err(msg) => {
                                    content = msg;
                                }
                            },
                            Err(msg) => {
                                println!("{}", msg);
                                return;
                            }
                        }

                        let content = format!(
                            "{{\n\"code\": \"{}\",\n\"msg\": \"{}\"\n}}",
                            content.0, content.1
                        );
                        response = format!(
                            "HTTP/1.1 200 OK\r\n\
                                Content-Type: text/html\r\n\
                                Content-Length: {}\r\n\r\n{}",
                            content.len(),
                            content
                        );

                        stream.write_all(response.as_bytes()).unwrap();
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
                                match api_call.version.as_str() {
                                    "v1" => {
                                        // get user by ID [/user/[uuid]/]
                                        match api_call.value[0].as_str() {
                                            "user" => {
                                                match str_req.0.as_str() {
                                                    "GET" => {
                                                        //TODO: Unsafe unwrap, might not be a
                                                        //UUID!!
                                                        let mut status_code = String::new();
                                                        let content = match Server::get_user(
                                                            &mut client,
                                                            Uuid::parse_str(&api_call.value[1])
                                                                .unwrap(),
                                                        ) {
                                                            Ok(user) => {
                                                                status_code = "200 OK".to_string();
                                                                user
                                                            }
                                                            Err(e) => {
                                                                status_code =
                                                                    "404 Not Found".to_string();
                                                                e
                                                            }
                                                        };

                                                        response = format!(
                                                            "HTTP/1.1 {}\r\n\
                                                                Content-Type: text/html\r\n\
                                                                Content-Length: {}\r\n\r\n{}",
                                                            status_code,
                                                            content.len(),
                                                            content
                                                        );

                                                        stream
                                                            .write_all(response.as_bytes())
                                                            .unwrap();
                                                    }
                                                    "POST" => match Server::extract_body(&req) {
                                                        Ok(body) => {
                                                            let mut content =
                                                                match Server::insert_user(
                                                                    &mut client,
                                                                    &body,
                                                                ) {
                                                                    Ok(user) => user,
                                                                    Err(msg) => msg,
                                                                };

                                                            response = format!(
                                                                        "HTTP/1.1 201 Created\r\n\
                                                                        Content-Type: text/html\r\n\
                                                                        Content-Length: {}\r\n\r\n{}",
                                                                        content.len(),
                                                                        content
                                                                    );

                                                            stream
                                                                .write_all(response.as_bytes())
                                                                .unwrap();
                                                        }
                                                        Err(e) => {}
                                                    },
                                                    //UPDATE
                                                    "PUT" => match Server::extract_body(&req) {
                                                        Ok(body) => {
                                                            let mut status_code = String::new();
                                                            let mut content =
                                                                match Server::update_user(
                                                                    &mut client,
                                                                    &body,
                                                                    Uuid::parse_str(
                                                                        &api_call.value[1],
                                                                    )
                                                                    .unwrap(),
                                                                ) {
                                                                    Ok(user) => {
                                                                        status_code =
                                                                            "200 OK".to_string();
                                                                        user
                                                                    }
                                                                    Err(e) => {
                                                                        status_code =
                                                                            "404 Not Found"
                                                                                .to_string();
                                                                        e
                                                                    }
                                                                };

                                                            response = format!(
                                                                        "HTTP/1.1 {}\r\n\
                                                                        Content-Type: text/html\r\n\
                                                                        Content-Length: {}\r\n\r\n{}",
                                                                        status_code,
                                                                        content.len(),
                                                                        content
                                                                    );

                                                            stream
                                                                .write_all(response.as_bytes())
                                                                .unwrap();
                                                        }
                                                        Err(msg) => {}
                                                    },
                                                    _ => {}
                                                }
                                            }
                                            "token" => {
                                                match str_req.0.as_str() {
                                                    "POST" => {
                                                        match Server::extract_body(&req) {
                                                            Ok(body) => {
                                                                let mut token = String::new();
                                                                let mut content =
                                                                    match Server::authenticate_user(
                                                                        &mut client,
                                                                        &body,
                                                                    ) {
                                                                        Ok(user) => {
                                                                            token = user.1;
                                                                            user.0
                                                                        }
                                                                        Err(msg) => msg,
                                                                    };

                                                                //TODO: what if user has cookies off?
                                                                response = format!(
                                                                        "HTTP/1.1 201 Created\r\n\
                                                                        Set-Cookie: token={}; Expires=Tue, 19 Jan 2038;  Secure; HttpOnly\r\n
                                                                        Content-Type: text/html\r\n\
                                                                        Content-Length: {}\r\n\r\n{}",
                                                                        token,
                                                                        content.len(),
                                                                        content
                                                                    );

                                                                stream
                                                                    .write_all(response.as_bytes())
                                                                    .unwrap();
                                                            }
                                                            Err(e) => {}
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            _ => {
                                                let content = format!(
                                                    "{{\n\"error\": \"Entity Not Found\"\n}}"
                                                );
                                                response = format!(
                                                    "HTTP/1.1 404 Not Found\r\n\
                                                        Content-Type: text/html\r\n\
                                                        Content-Length: {}\r\n\r\n{}",
                                                    content.len(),
                                                    content
                                                );

                                                stream.write_all(response.as_bytes()).unwrap();
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            None => {
                                match Server::get_extension(&str_req.1) {
                                    Ok(ext) => {
                                        let mut content = Vec::new();
                                        match ext {
                                            //TODO: Consider stripping evil things like '../..' from
                                            // the requested resource.
                                            "svg" => {
                                                match Server::get_file(
                                                    format!("/rsrcs/{}", str_req.1).as_str(),
                                                ) {
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

                                                stream.write_all(&byte_res).unwrap();
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => {
                                        let content = match Server::get_page("/views/whats-that.html") {
                                        Ok(html) => html,
                                        Err(e) => format!("<html><body>Webpage was not formatted correctly, please @funnymania_ in case they are sleeping (Zzzz):<br><a href=\"https://twitter.com/funnymania_\">https://twitter.com/funnymania_</a><br><br>Error: {}</body></html>", e)
                                    };

                                        response = format!(
                                            "HTTP/1.1 404 Not Found\r\n\
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

        stream.flush().unwrap();
    }

    pub fn run(port: u64, port_http: u64) {
        let mut address = String::from("0.0.0.0:");
        address += &port.to_string();
        let listener = TcpListener::bind(&address).unwrap();

        let mut address_http = String::from("0.0.0.0:");
        address_http += &port_http.to_string();
        let listener_http = TcpListener::bind(&address_http).unwrap();

        // Start postgres client
        let mut db_client = Arc::new(Mutex::new(
            Client::connect("host=localhost user=postgres", NoTls).unwrap(),
        ));

        // open Log file
        let mut log_file = Arc::new(Server::tail_file().unwrap());

        let mut th_db_client_http = db_client.clone();
        let th_log_file_http = log_file.clone();

        // Prep SSL Stream
        #[cfg(feature = "https")]
        {
            let mut ssl_key = format!("{}", env!("CARGO_MANIFEST_DIR"));
            ssl_key += "/keys/privkey.pem";

            let mut ssl_chain = format!("{}", env!("CARGO_MANIFEST_DIR"));
            ssl_chain += "/keys/fullchain.pem";

            let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            acceptor
                .set_private_key_file(ssl_key.as_str(), SslFiletype::PEM)
                .unwrap();
            acceptor
                .set_certificate_chain_file(ssl_chain.as_str())
                .unwrap();
            acceptor.check_private_key().unwrap();
            let acceptor = Arc::new(acceptor.build());

            let acceptor = acceptor.clone();
            let mut th_db_client = db_client.clone();
            let th_log_file = log_file.clone();

            let https_join_handle = thread::spawn(move || {
                for stream in listener.incoming() {
                    let mut th_db_client = th_db_client.clone();
                    let th_log_file = th_log_file.clone();
                    match stream {
                        Ok(mut stream) => {
                            println!("Https Peer at {:?}", stream.peer_addr());
                            let stream_c = stream.try_clone().unwrap();
                            match acceptor.accept(stream_c) {
                                Ok(stream_a) => {
                                    thread::spawn(move || {
                                        Server::handle_https(stream_a, th_log_file, th_db_client);
                                    });
                                }
                                Err(e) => {
                                    println!("{:?}", e);
                                    let content = String::from(
                                        "This port can only be used for HTTPS (not HTTP)",
                                    );
                                    let response = format!(
                                        "HTTP/1.1 418 I'm a teapot\r\n\
                                Content-Length: {}\r\n\r\n{}",
                                        content.len(),
                                        content
                                    );
                                    stream.write(response.as_bytes()).unwrap();
                                    stream.flush().unwrap();
                                }
                            }
                        }
                        Err(e) => println!("TcpStream error: {}", e),
                    }
                }
            });

            // println!("{:?}", https_join_handle.join());
        }

        let http_join_handle = thread::spawn(move || {
            for stream in listener_http.incoming() {
                let th_log_file_http = th_log_file_http.clone();
                let mut th_db_client_http = th_db_client_http.clone();
                match stream {
                    Ok(stream) => {
                        println!("Http Peer from {:?}", stream.peer_addr());
                        thread::spawn(move || {
                            Server::handle_https(stream, th_log_file_http, th_db_client_http);
                        });
                    }
                    Err(e) => println!("TcpStream error: {}", e),
                }
            }
        });

        println!("{:?}", http_join_handle.join());
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

    /// Only GET, POST, and PUT valid atm
    pub fn whats_reqd(req: String) -> (String, String) {
        let mut lines: Vec<&str> = req.lines().collect();
        let first_line: Vec<&str> = lines[0].split(" ").collect();
        println!("What is this: {}", first_line[0]);
        match first_line[0] {
            "GET" => (String::from("GET"), String::from(first_line[1])),
            "POST" => (String::from("POST"), String::from(first_line[1])),
            "PUT" => (String::from("PUT"), String::from(first_line[1])),
            _ => {
                if first_line.len() < 2 {
                    (String::from(""), String::from(""))
                } else {
                    (String::from(""), String::from(first_line[1]))
                }
            }
        }
    }

    pub fn get_installs(client: &mut Arc<Mutex<postgres::Client>>) -> Result<i64, String> {
        let mut client = client.lock().unwrap();
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
            // println!("{}", &path[left..right]);
            Ok(&path[left..right])
        }
    }

    pub fn tail_file() -> io::Result<fs::File> {
        let mut file_path = format!("{}", env!("CARGO_MANIFEST_DIR"));
        file_path += "/logs/tail.txt";
        let mut file = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(file_path);

        file
    }

    pub fn add_to_file(mut file: &fs::File, req: &[u8]) {
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        if contents.len() + req.len() < 4096 {
            file.write(req);
        }
    }

    pub fn add_subscriber<'a>(
        client: &mut Arc<Mutex<postgres::Client>>,
        email: &str,
    ) -> Result<String, String> {
        //generate unique ID
        let uuid = &Uuid::new_v4();

        let mut client = client.lock().unwrap();
        let res = client.query("INSERT INTO subscriber VALUES($1, $2)", &[&uuid, &email]);
        match res {
            Ok(rows) => Ok(String::from("Success")),
            Err(e) => Err(format!("{}", e.code().unwrap().code())),
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
                if req[i] == 10 && req[i - 1] == 13 && req[i - 2] == 10 && req[i - 1] == 13 {
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
            return Err((
                String::from("Email Format"),
                String::from("Email must be shorter than 256 characters"),
            ));
        }

        let email_parts = Server::email_address_parts(email);
        println!(
            "email: {} {} {} {}",
            email, email_parts.0, email_parts.1, email_parts.2
        );
        if email_parts.0 == "" || email_parts.1 == "" || email_parts.2 == "" {
            return Err((
                String::from("Email Format"),
                String::from("Email must be in the proper format: you@example.com"),
            ));
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

    pub fn get_user(
        client: &mut Arc<Mutex<postgres::Client>>,
        uid: uuid::Uuid,
    ) -> Result<json::JsonValue, json::JsonValue> {
        let mut client = client.lock().unwrap();
        let res = client.query("SELECT * FROM users WHERE id = $1", &[&uid]);
        match res {
            Ok(rows) => {
                if rows.len() == 0 {
                    Err(object! {
                        error: "UserId not found".to_string()
                    })
                } else {
                    Ok(object! {
                        user: {
                            id: rows[0].get::<&str, Uuid>("id").to_simple().to_string(),
                            name: rows[0].get::<&str, String>("name")
                        }
                    })
                }
            }
            Err(e) => Err(object! {error: e.code().unwrap().code()}),
        }
    }

    pub fn update_user(
        client: &mut Arc<Mutex<postgres::Client>>,
        json_user: &str,
        uid: uuid::Uuid,
    ) -> Result<json::JsonValue, json::JsonValue> {
        let user_obj = json::parse(json_user).unwrap();
        let name_plain = user_obj["user"]["name"].as_str();
        if name_plain == None {
            return Err(object! {error: String::from("'Name' field must be a string")});
        }

        let mut client = client.lock().unwrap();
        if user_obj["user"]["name"].is_string() {
            let res = client.query(
                "UPDATE users SET name = $1 WHERE id = $2 RETURNING *",
                &[&name_plain, &uid],
            );
            match res {
                Ok(rows) => {
                    if rows.len() == 0 {
                        Err(object! {
                            error: "UserId not found".to_string()
                        })
                    } else {
                        println!("{:?}", rows[0]);
                        Ok(object! {
                            user: {
                                id: rows[0].get::<&str, Uuid>("id").to_simple().to_string(),
                                name: rows[0].get::<&str, String>("name")
                            }
                        })
                    }
                }
                Err(e) => Err(object! {error: e.code().unwrap().code()}),
            }
        } else {
            Err(object! {error: String::from("'Name' field must be a string")})
        }
    }

    pub fn insert_user(
        client: &mut Arc<Mutex<postgres::Client>>,
        json_user: &str,
    ) -> Result<json::JsonValue, json::JsonValue> {
        let mut uid = Uuid::new_v4();
        let mut client = client.lock().unwrap();
        loop {
            if client
                .query("SELECT * FROM users WHERE id = $1", &[&uid])
                .unwrap()
                .len()
                == 1
            {
                uid = Uuid::new_v4();
                continue;
            }

            break;
        }

        let user_obj = json::parse(json_user).unwrap();
        let name_plain = user_obj["user"]["name"].as_str();
        if name_plain == None {
            return Err(object! {error: String::from("'Name' field must be a string")});
        }

        if user_obj["user"]["name"].is_string() {
            let res = client.query(
                "INSERT INTO users (id, name) VALUES ($1, $2) RETURNING *",
                &[&uid, &name_plain],
            );
            match res {
                Ok(rows) => {
                    if rows.len() == 0 {
                        Err(object! {
                            error: "UserId not found".to_string()
                        })
                    } else {
                        Ok(object! {
                            user: {
                                id: rows[0].get::<&str, Uuid>("id").to_simple().to_string(),
                                name: rows[0].get::<&str, String>("name")
                            }
                        })
                    }
                }
                Err(e) => Err(object! {error: e.code().unwrap().code()}),
            }
        } else {
            Err(object! {error: String::from("'Name' field must be a string")})
        }
    }

    ///Return ApiString if it is an api call, otherwise None
    pub fn is_valid_api(path: &str) -> Option<ApiString> {
        // Break it up by '/' characters
        let split: Vec<&str> = path.split('/').collect();

        if split.len() == 0 {
            return None;
        }

        // First str should be v#
        if !Server::is_version_str(split[0]) {
            return None;
        }

        let mut value = Vec::new();
        for i in 1..split.len() {
            if split[i] != "" {
                value.push(split[i].to_string());
            } else {
                return None;
            }
        }

        Some(ApiString {
            version: split[0].to_string(),
            value,
        })
    }

    // Checks if valid version string is present ex: "v2", "v12". "v2.0" is invalid.
    pub fn is_version_str(test: &str) -> bool {
        let test_bytes = test.as_bytes();
        if test_bytes.len() < 2 {
            return false;
        }

        if test_bytes[0] as char != 'v' {
            return false;
        }

        // Is test[therest] a number
        if test_bytes[1] as char == '0' {
            return false;
        }

        match test[1..].parse::<u32>() {
            Ok(_) => true,
            _ => false,
        }
    }

    //TODO: Multi-device login
    pub fn authenticate_user(
        client: &mut Arc<Mutex<postgres::Client>>,
        body: &str,
    ) -> Result<(json::JsonValue, String), json::JsonValue> {
        //      Extract pass, hash it
        let user_obj = json::parse(body).unwrap();
        let name_plain = user_obj["user"]["name"].as_str();
        if name_plain == None {
            return Err(object! {error: String::from("'Name' field must be a string")});
        }

        let pass_plain = user_obj["user"]["pass"].as_str();
        if pass_plain == None {
            return Err(object! {error: String::from("'Password' is in an invalid format")});
        }

        let hashed_pass = Server::hash(pass_plain.unwrap());
        let mut client = client.lock().unwrap();

        if user_obj["user"]["name"].is_string() && user_obj["user"]["pass"].is_string() {
            //      Select user if they match
            let res = client.query(
                "SELECT * from user WHERE name = $1 AND pass = $2",
                &[&name_plain, &hashed_pass],
            );

            match res {
                Ok(rows) => {
                    if rows.len() == 0 {
                        Err(object! {
                            error: "User and pass not found. Try again.".to_string()
                        })
                    } else {
                        let uid = rows[0].get::<&str, Uuid>("id").to_simple().to_string();
                        let other_logins =
                            client.query("SELECT * FROM user_device WHERE uid = $1", &[&uid]);
                        match other_logins {
                            Ok(token_rows) => {
                                let new_id = Uuid::new_v4();
                                //TODO: Default ID (unknown), 1 = iphone, 2 = android, 3 = mac,
                                //4 = linux, 5 = windows
                                let new_sess_token = Uuid::new_v4();
                                let insert_res = client.query("INSERT INTO user_device (uid, id, device_type, sess_token, sess_date) VALUES ($1, $2, $3, $4, now) RETURNING *", &[&uid, &new_id, &new_sess_token]);
                                match insert_res {
                                    Ok(rows) => Ok((
                                        object! {
                                            user: {
                                                id: rows[0].get::<&str, Uuid>("id").to_simple().to_string(),
                                                name: rows[0].get::<&str, String>("name")
                                            }
                                        },
                                        new_sess_token.to_string(),
                                    )),
                                    Err(e) => Err(object! {
                                        error: "Could not authenticate user".to_string()
                                    }),
                                }
                            }
                            Err(e) => Err(object! {
                                error: "Could not authenticate user".to_string()
                            }),
                        }
                    }
                }
                Err(e) => Err(object! {error: e.code().unwrap().code()}),
            }
        } else {
            Err(object! {error: String::from("'Name' field must be a string")})
        }
    }

    //TODO: Finish hash.
    pub fn hash(browns: &str) -> String {
        let result = String::new();

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    //TODO: Get table structure (column names, qualities) from test_users before dropping it.
    //TODO: Establish how to add columns to users in general.
    fn setup() -> postgres::Client {
        // Start postgres client
        let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

        // Remove test_tables
        client.execute("DROP TABLE users", &[]);

        // Create create test_tables
        client.execute(
            "CREATE TABLE users (id uuid PRIMARY KEY, name varchar(255))",
            &[],
        );

        client
    }

    #[test]
    fn insert() {
        let mut client = setup();

        let json_user = "{ \"user\": { \"name\": \"Kurt\", \"id\": \"00000000-0000-0000-0000-000000000000\" } }";
        match Server::insert_user(&mut client, json_user) {
            Ok(result) => {
                assert_eq!(result["user"]["name"].as_str().unwrap(), "Kurt");
            }
            Err(e) => {
                panic!("{}", e)
            }
        }
    }

    #[test]
    fn update_id_not_found() {
        let mut client = setup();

        let uid = Uuid::new_v4();

        let json_user = "{ \"user\": { \"name\": \"Kurt\", \"id\": \"00000000-0000-0000-0000-000000000000\" } }";

        match Server::update_user(&mut client, json_user, uid) {
            Err(result) => {
                assert_eq!(
                    result,
                    object! {
                        error: "UserId not found".to_string()
                    }
                );
            }
            _ => {
                panic!("UserID 00000000-0000-0000-0000-000000000000 should not be found.")
            }
        }
    }

    #[test]
    fn select_id_not_found() {
        let mut client = setup();

        let uid = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();

        let json_user = "{ \"user\": { \"name\": \"Kurt\", \"id\": \"00000000-0000-0000-0000-000000000000\" } }";

        match Server::get_user(&mut client, uid) {
            Err(result) => {
                assert_eq!(
                    result,
                    object! {
                        error: "UserId not found".to_string()
                    }
                );
            }
            _ => {
                panic!("UserID 00000000-0000-0000-0000-000000000000 should not be found.")
            }
        }
    }

    #[test]
    fn select_user() {
        // Insert user.
        let mut client = setup();

        let json_user = "{ \"user\": { \"name\": \"Kurt\", \"id\": \"00000000-0000-0000-0000-000000000000\" } }";

        // Query for that user.
        match Server::insert_user(&mut client, json_user) {
            Ok(result) => {
                let uid = Uuid::parse_str(result["user"]["id"].as_str().unwrap()).unwrap();
                match Server::get_user(&mut client, uid) {
                    Ok(user) => {
                        assert_eq!(
                            user["user"]["id"].as_str().unwrap(),
                            uid.to_simple().to_string()
                        );
                        assert_eq!(user["user"]["name"], result["user"]["name"]);
                    }
                    Err(e) => panic!("{}", e),
                }
            }
            Err(e) => {
                panic!("{}", e)
            }
        }
    }

    #[test]
    fn update_user() {
        // Insert user.
        let mut client = setup();

        let json_user = "{ \"user\": { \"name\": \"Kurt\", \"id\": \"00000000-0000-0000-0000-000000000000\" } }";

        // Query for that user.
        match Server::insert_user(&mut client, json_user) {
            Ok(result) => {
                let uid = Uuid::parse_str(result["user"]["id"].as_str().unwrap()).unwrap();
                let changed_user = "{ \"user\": { \"name\": \"Cheri\", \"id\": \"00000000-0000-0000-0000-000000000000\" } }";
                match Server::update_user(&mut client, json_user, uid) {
                    Ok(user) => {
                        assert_eq!(
                            user["user"]["id"].as_str().unwrap(),
                            uid.to_simple().to_string()
                        );
                        assert_eq!(user["user"]["name"], result["user"]["name"]);
                    }
                    Err(e) => panic!("{}", e),
                }
            }
            Err(e) => {
                panic!("{}", e)
            }
        }
    }
}
