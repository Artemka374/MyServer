use crate::db;
use crate::server_error::ServerError;
use crate::CurrPgPool;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use serde_json::json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct NoteQuery {
    pub name: String,
    pub text: String,
}

#[get("/notes/{id}")]
pub async fn find(
    db: web::Data<CurrPgPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, ServerError> {
    let mut conn = db::connection(&db.pool).await?;
    let note = db::find(&mut conn, id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(note))
}

#[get("/notes")]
pub async fn pagination(
    db: web::Data<CurrPgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, ServerError> {
    let req = Some(req.query_string());

    let mut params: HashMap<String, String> = req
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);

    params
        .entry(String::from("page"))
        .or_insert(String::from("0"));
    params
        .entry(String::from("size"))
        .or_insert(String::from("10"));
    params.entry(String::from("q")).or_insert(String::new());

    let page = params.get(&String::from("page")).unwrap();
    let size = params.get(&String::from("size")).unwrap();
    let q = params.get(&String::from("q")).unwrap();

    let page = page.parse::<usize>().unwrap();
    let size = size.parse::<usize>().unwrap();

    let mut conn = db::connection(&db.pool).await?;

    let notes = db::filter(&mut conn, page, size, q.clone()).await?;

    Ok(HttpResponse::Ok().json(notes))
}

#[post("/notes")]
pub async fn create(
    db: web::Data<CurrPgPool>,
    note: web::Json<NoteQuery>,
) -> Result<HttpResponse, ServerError> {
    let mut conn = db::connection(&db.pool).await?;
    let note = db::create(&mut conn, note.into_inner()).await?;
    Ok(HttpResponse::Ok().json(json!({ "ID": note })))
}

#[put("/notes/{id}")]
pub async fn update(
    db: web::Data<CurrPgPool>,
    id: web::Path<i32>,
    note: web::Json<NoteQuery>,
) -> Result<HttpResponse, ServerError> {
    let mut conn = db::connection(&db.pool).await?;
    let note = db::update(&mut conn, id.into_inner(), note.into_inner()).await?;
    Ok(HttpResponse::Ok().json(note))
}

#[delete("/notes/{id}")]
pub async fn delete(
    db: web::Data<CurrPgPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, ServerError> {
    let mut conn = db::connection(&db.pool).await?;
    let deleted_note = db::delete(&mut conn, id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(json!({ "success": deleted_note })))
}
