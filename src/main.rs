mod endpoints;
mod secrets;
mod verification;
mod email;
mod whitelist;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(endpoints::index))
            .route("/send-sta", web::post().to(endpoints::send_sta))
            .route("/send-written", web::post().to(endpoints::send_written))
            .route("/verify/{code}", web::get().to(endpoints::verify))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}