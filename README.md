# Abode's Launch Page
Flexible implementation for an API or web server. Supports postgres, https, and email sending. Really fast. 

## Development Build
cargo run d

Https defaults to true, which requires key files to be included to compile.

## Production Build
cargo build --release

We serve this via systemd units on AWS ec2 micros. Feel free to fork for your own purpose. 
Take note that AWS will not actually support the email function on their EC2 intances (unless you can convince them otherwise)

## Environment

For emailing and https: You need Openssl installed. Please refer (here)[https://docs.rs/openssl/0.10.37/openssl/#automatic] for directions on how openssl-rust searches for Openssl's libs and headers.
