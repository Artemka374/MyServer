use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;

mod api;
mod db;
mod server_error;

pub struct PgPoolData {
    pub pool: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::init().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .data(PgPoolData { pool: pool.clone() })
            .service(api::find)
            .service(api::delete)
            .service(api::update)
            .service(api::create)
            .service(api::pagination)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
