use super::super::AppState;
use crate::models::user_models::*;
use actix_session::Session;
use actix_web::{
    error::Error,
    error::{ErrorForbidden, ErrorInternalServerError, ErrorNotFound},
    http::StatusCode,
    web::Data,
    HttpResponse,
};

pub const SESSION_NAME: &str = "jack-x-auth";

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

pub fn get_sessid(sess: Session) -> Result<i64, Error> {
    return match sess.get(SESSION_NAME)? {
        Some(s) => Ok(s),
        None => Err(ErrorForbidden("could not find user_id")),
    };
}

pub fn get_sessuser(state: &Data<AppState>, sess: Session) -> Result<User, Error> {
    let user_id = match get_sessid(sess) {
        Ok(id) => id,
        Err(e) => return Err(e),
    };

    let user: User = match state.first_sql("SELECT * FROM user WHERE id = ?", (user_id,))? {
        None => return Err(ErrorInternalServerError("could not find user")),
        Some(u) => u,
    };
    return Ok(user);
}
