use actix_web::{App, HttpResponse, HttpServer, Responder, get};

mod generate_pictures;
mod hex2color;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub struct Size {
    width: u32,
    height: u32
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Thanks for using setu-api!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
