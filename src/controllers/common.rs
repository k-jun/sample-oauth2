use super::super::AppState;
use crate::models::todo_models::*;
use actix_web::{
    error::Error,
    http::StatusCode,
    web::{Data, Json, Path},
    HttpResponse,
};
#[macro_use]
use serde_json::json;

pub fn failure(status: u16) -> HttpResponse {
    let status = StatusCode::from_u16(status).expect("invalide status given");
    HttpResponse::build(status).finish()
}

pub fn success(status: u16, json_str: mysql::serde_json::Value) -> HttpResponse {
    let status = StatusCode::from_u16(status).expect("invalide status given");
    HttpResponse::build(status)
        .content_type("application/json")
        .json(json_str)
}
