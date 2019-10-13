use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    fname: String,
    lname: String,
    street: String,
    city: String,
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn display_user(form: web::Form<FormData>) -> impl Responder {
    if form.fname.len() > 30 || form.lname.len() > 30 || form.street.len() > 30 || form.city.len() > 30 {
        return HttpResponse::BadRequest().body("Input too long")
    }

    let data = format!(
        "Hello, {} {}! You live in {} at {}.",
        form.fname.to_uppercase(),
        form.lname.to_uppercase(),
        form.city.to_uppercase(),
        form.street.to_uppercase()
    );
    HttpResponse::Ok().content_type("text/html").body(data)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(index))
            .route("/show_user", web::post().to(display_user))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8081")
    .unwrap()
    .run()
    .unwrap();
}
