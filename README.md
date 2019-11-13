# Weird XSS
A vulnerable webserver to XSS

## Running
Get `rustup` to get the Rust toolchain: [Install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html).
Then generate a certificate so the server can run on HTTPS with `generate_ssl.sh`. Note that this requires openssl.
Finally, do `cargo run` to start the webserver running on `127.0.0.1:8088`.

## I give up
First of all, try again. You can do this!

If you truly want to see the solution, I have provided a solution folder with
almost all that you need to pop the XSS. With that, you should be able to figure
out the rest :)
