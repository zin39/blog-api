use actix_web::{get, http, App, HttpResponse, HttpServer, Responder};
use diesel::{Connection, PgConnection};
use tracing::info;

#[get("/health")]
async fn health_check() -> impl Responder {
    info!("Health check requested");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    match PgConnection::establish(&db_url) {
        Ok(_) => HttpResponse::Ok().body("OK"),
        Err(e) => {
            HttpResponse::ServiceUnavailable().body(format!("Database connection failed: {}", e))
        }
    }
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    HttpServer::new(|| App::new().service(health_check))
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
