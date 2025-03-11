use actix_web::{web, App, HttpServer, Responder};

async fn hola() -> impl Responder {
    "¡Hola, mundo!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hola", web::get().to(hola))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
