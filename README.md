# Abode's Launch Page
Webserver for Abode's launch page. Includes postgres support. Really fast. 

## Development Build
cargo run d

## Dev + Email Feature
cargo run --features email d

Note that the 'd' argument must come _after_ FEATURE flag.

## Production Build
cargo build --release

We serve this via systemd units on AWS ec2 micros. Feel free to fork for your own purpose. 

## Environment

For emailing: You need Openssl installed. Please refer here for directions on how openssl-rust looks for Openssl's libs and headers.

