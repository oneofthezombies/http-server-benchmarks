use std::env;

use ntex::web;

#[web::get("/")]
async fn hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hello, World!")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();
    web::HttpServer::new(|| web::App::new().service(hello))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
