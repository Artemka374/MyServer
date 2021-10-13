use crate::api::NoteQuery;
use crate::server_error::ServerError;
use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::postgres::PgPool;
use std::env;

pub type PoolConn = sqlx::pool::PoolConnection<sqlx::Postgres>;

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub id: i32,
    pub name: String,
    pub text: String,
}

pub async fn connection(pool: &PgPool) -> Result<PoolConn, ServerError> {
    PgPool::acquire(pool)
        .await
        .map_err(|e| ServerError::new(500, format!("Failed getting database connection: {}", e)))
}

pub async fn init() -> Result<PgPool, ServerError> {
    let db_url = env::var("DATABASE_URL").expect("Database url must be set!");
    let pool = PgPool::connect(&db_url).await.unwrap();
    Ok(pool)
}

pub async fn create(conn: &mut PoolConn, note: NoteQuery) -> Result<i32, ServerError> {
    let sql_response = sqlx::query!(
        r#"
        INSERT INTO notes (name, text) VALUES ($1, $2)
        RETURNING id"#,
        note.name,
        note.text
    )
    .fetch_one(conn)
    .await?;

    Ok(sql_response.id)
}

pub async fn filter(
    conn: &mut PoolConn,
    page: usize,
    size: usize,
    q: String,
) -> Result<Vec<Note>, ServerError> {
    let mut query_str = "%".to_owned();
    query_str.push_str(q.as_str());
    query_str.push_str("%");

    let offset: i64 = (page * size) as i64;
    let limit: i64 = size as i64;

    let sql_response = sqlx::query!(
        r#"
            SELECT id, name, text
            FROM notes
            WHERE name LIKE $1
            ORDER BY id
            OFFSET $2
            LIMIT $3;
        "#,
        &query_str,
        offset,
        limit
    )
    .fetch_all(conn)
    .await?;

    let mut result = Vec::new();

    for i in 0..sql_response.len() {
        result.push(Note {
            id: sql_response.get(i).unwrap().id,
            name: sql_response.get(i).unwrap().name.clone(),
            text: sql_response.get(i).unwrap().text.clone(),
        });
    }

    Ok(result)
}

pub async fn delete(conn: &mut PoolConn, id: i32) -> Result<i32, ServerError> {
    let sql_response = sqlx::query!(
        r#"
        DELETE FROM notes
        WHERE id = $1
        RETURNING id;"#,
        id
    )
    .fetch_one(conn)
    .await?;
    Ok(sql_response.id)
}

pub async fn update(
    conn: &mut PoolConn,
    id: i32,
    note: NoteQuery,
) -> Result<i32, ServerError> {
    let sql_response = sqlx::query!(
        r#"
        UPDATE notes
        SET (name, text) = ($1, $2)
        WHERE id = $3
        RETURNING id;"#,
        note.name,
        note.text,
        id
    )
    .fetch_one(conn)
    .await?;

    Ok(sql_response.id)
}

pub async fn find(conn: &mut PoolConn, id: i32) -> Result<Note, ServerError> {
    let sql_response = sqlx::query!(
        r#"
        SELECT name, text
        FROM notes
        WHERE id = $1;"#,
        id
    )
    .fetch_one(conn)
    .await?;
    Ok(Note {
        id,
        name: sql_response.name,
        text: sql_response.text,
    })
}
