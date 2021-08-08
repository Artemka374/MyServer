use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx;
use sqlx::postgres::PgPool;
mod api;
mod db;
mod server_error;

pub struct CurrPgPool {
    pub pool: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::init().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .data(CurrPgPool { pool: pool.clone() })
            .service(routes::find)
            .service(routes::delete)
            .service(routes::update)
            .service(routes::create)
            .service(routes::pagination)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
