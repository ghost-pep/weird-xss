# Weird XSS
A vulnerable webserver to XSS

## Running
Get `rustup` to get the Rust toolchain: [Install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html).
Then generate a certificate so the server can run on HTTPS with `generate_ssl.sh`. Note that this requires openssl.
Finally, do `cargo run` to start the webserver running on `127.0.0.1:8088`.
