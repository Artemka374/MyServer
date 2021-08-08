use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Deserialize;
use serde_json::json;
use sqlx::Error as SQLError;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct ServerError {
    status_code: u16,
    error_message: String,
}

impl ServerError {
    pub fn new(status_code: u16, error_message: String) -> ServerError {
        ServerError {
            status_code,
            error_message,
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.error_message.as_str())
    }
}

impl From<SQLError> for ServerError {
    fn from(error: SQLError) -> ServerError {
        match error {
            SQLError::RowNotFound => ServerError::new(404, "Note not found".to_string()),
            SQLError::Database(err) => ServerError::new(409, err.message().to_string()),
            err => ServerError::new(500, format!("Unknown database error: {}", err)),
        }
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = match self.status_code < 500 {
            true => self.error_message.clone(),
            false => "Internal server error".to_string(),
        };

        HttpResponse::build(status_code).json(json!({ "message": error_message }))
    }
}
