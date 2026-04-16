use actix_web::{App, HttpServer, web};
mod routes;

use routes::healthz;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/app").service(healthz)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
