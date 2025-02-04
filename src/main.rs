use actix_web::{web, App, HttpServer};
use blog_api::{establish_connection_pool, health_check};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    let pool = establish_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(health_check)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

