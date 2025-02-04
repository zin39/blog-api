use actix_web::{get, web, HttpResponse, Responder};
// In your lib.rs or a dedicated module for database setup
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

// In your health_check endpoint:
#[get("/health")]
pub async fn health_check(pool: web::Data<PgPool>) -> impl Responder {
    tracing::info!("Health check requested");
    match pool.get() {
        Ok(_) => HttpResponse::Ok().body("OK"),
        Err(e) => {
            HttpResponse::ServiceUnavailable().body(format!("Database connection failed: {}", e))
        }
    }
}

