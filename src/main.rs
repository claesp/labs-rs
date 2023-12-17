use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "labs.cx.se"
}

#[get("/tools")]
async fn tools() -> impl Responder {
    "labs.cx.se/tools"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(tools))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
