use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx;
use sqlx::postgres::PgPool;
mod db;
mod public_api;
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
            .service(public_api::find)
            .service(public_api::delete)
            .service(public_api::update)
            .service(public_api::create)
            .service(public_api::pagination)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
