use actix_web::{web::Json, HttpResponse};
use serde::Serialize;

use crate::errors::ApiError;

pub fn respond_json<T>(data: T) -> Result<Json<T>, ApiError>
where
    T: Serialize,
{
    Ok(Json(data))
}

pub fn respond_ok() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body("ok"))
}
