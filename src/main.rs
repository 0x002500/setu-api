use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use serde::Deserialize;
use crate::{generate_pictures::generate_pictures, hex2color::hex2color};

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

#[derive(Deserialize)]
struct Pic {
    color: String,
    width: u32,
    height: u32
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Thanks for using setu-api!")
}

#[get("/pic")]
async fn pic(pic: web::Query<Pic>) -> impl Responder {
    let size: Size = Size { width: pic.width, height: pic.height };
    let color: Color = hex2color(&pic.color);
    let image_bytes: Vec<u8> = generate_pictures(color, size).await;
    HttpResponse::Ok().body(image_bytes)
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
