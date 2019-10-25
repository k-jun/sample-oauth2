use super::super::AppState;
use crate::models::client_token_models::*;
use actix_web::{
    error::Error,
    error::{ErrorForbidden, ErrorNotFound},
    web::{Data, Json, Path},
    HttpResponse,
};
#[macro_use]
use serde_json::json;
use crate::controllers::common::*;
use actix_session::Session;

fn get_ctoken(state: &Data<AppState>, ctoken_id: String) -> Result<ClientToken, Error> {
    match state.first_sql("SELECT * FROM client_token WHERE id = ?", (ctoken_id,))? {
        None => Err(ErrorNotFound("could not find ctoken")),
        Some(t) => Ok(t),
    }
}

pub fn index(state: Data<AppState>, sess: Session) -> Result<HttpResponse, Error> {
    let user = match get_sessuser(&state, sess) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };
    if user.authority_level < 900 {
        return Err(ErrorForbidden("could not do this operation"));
    }

    let tokens: Vec<ClientToken> =
        state.query_sql("SELECT * FROM client_token WHERE user_id = ?", (user.id,))?;
    Ok(success(
        200,
        json!({ "client_tokens": tokens
            .iter()
            .map(|token| {
                json!({
                    "id": token.id,
                    "user_id": token.user_id,
                    "client_token": token.token,
                })
            }).collect::<Vec<mysql::serde_json::Value>>() }),
    ))
}

pub fn read(
    state: Data<AppState>,
    path: Path<ClientTokenPath>,
    sess: Session,
) -> Result<HttpResponse, Error> {
    let user = match get_sessuser(&state, sess) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };
    if user.authority_level < 500 {
        return Ok(failure(403));
    }

    let token: ClientToken =
        match state.first_sql("SELECT * FROM client_token WHERE id = ?", (&path.id,))? {
            None => return Ok(failure(404)),
            Some(v) => v,
        };

    Ok(success(
        200,
        json!({
          "id": token.id,
          "user_id": token.user_id,
          "token": token.token,
          "secret_token": token.secret_token,
        }),
    ))
}

pub fn create(state: Data<AppState>, sess: Session) -> Result<HttpResponse, Error> {
    let user = match get_sessuser(&state, sess) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };
    if user.authority_level < 500 {
        return Ok(failure(403));
    }

    let client_token_id = state.exec_sql_insert_id(
        "INSERT INTO client_token (user_id, token, secret_token) VALUES (?, SUBSTRING(MD5(RAND()), 1, 10), SUBSTRING(MD5(RAND()), 1, 10))",
        (&user.id,),
    )?;

    Ok(success(200, json!({ "client_token_id": client_token_id })))
}

pub fn delete(
    state: Data<AppState>,
    path: Path<ClientTokenPath>,
    sess: Session,
) -> Result<HttpResponse, Error> {
    let user = match get_sessuser(&state, sess) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };

    let t_ctoken = match get_ctoken(&state, path.id.to_string()) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };

    if user.id != t_ctoken.user_id {
        return Ok(failure(403));
    }

    state.exec_sql("DELETE FROM client_token WHERE id = ?", (&path.id,))?;

    Ok(success(200, json!("")))
}
