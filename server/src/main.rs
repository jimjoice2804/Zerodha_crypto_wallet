use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
mod routes;

use routes::healthz;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/app").service(healthz))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
