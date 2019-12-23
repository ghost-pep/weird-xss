use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[derive(Deserialize)]
struct FormData {
    fname: String,
    lname: String,
    city: String,
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn display_user(form: web::Form<FormData>) -> impl Responder {
    if form.fname.len() > 24 || form.lname.len() > 24 || form.city.len() > 24 {
        return HttpResponse::BadRequest().body("Input too long")
    }

    let data = format!(
        "Hello, {} {}! You live in {}.",
        form.fname.to_uppercase(),
        form.lname.to_uppercase(),
        form.city.to_uppercase(),
    );
    HttpResponse::Ok().content_type("text/html").body(data)
}

fn main() {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(index))
            .route("/show_user", web::post().to(display_user))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind_ssl("0.0.0.0:8088", builder)
    .unwrap()
    .run()
    .unwrap();
}
