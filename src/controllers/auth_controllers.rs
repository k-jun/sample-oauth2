use super::super::AppState;
use crate::models::access_token_models::*;
use crate::models::auth_models::*;
use crate::models::client_token_models::*;
use crate::models::temp_token_models::*;
use actix_web::{
    error::Error,
    error::{ErrorForbidden, ErrorInternalServerError, ErrorNotFound},
    web::{Data, Json, Path, Query},
    HttpResponse,
};
#[macro_use]
use serde_json::json;
use crate::controllers::common::*;
use actix_session::Session;

fn get_client_token(state: &Data<AppState>, client_token: String) -> Result<ClientToken, Error> {
    match state.first_sql(
        "SELECT * FROM client_token WHERE token = ?",
        (client_token,),
    )? {
        None => Err(ErrorNotFound("could not find client_token")),
        Some(t) => Ok(t),
    }
}

fn get_temp_token(state: &Data<AppState>, token: String) -> Result<TempToken, Error> {
    match state.first_sql("SELECT * FROM temp_token WHERE token = ?", (token,))? {
        None => Err(ErrorNotFound("could not find temp_token")),
        Some(t) => Ok(t),
    }
}

pub fn first_flow(
    state: Data<AppState>,
    query: Query<FirstFlowQuery>,
    sess: Session,
) -> Result<HttpResponse, Error> {
    let user = match get_sessuser(&state, sess) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };
    // TODO トランザクション
    let client_token: ClientToken = match get_client_token(&state, query.client_token.to_string()) {
        Err(v) => return Err(v),
        Ok(v) => v,
    };

    if client_token.secret_token != query.client_secret_token {
        return Err(ErrorForbidden("Bad Parameters"));
    }

    let temp_token: TempToken = match state.first_sql(
        "SELECT * FROM temp_token WHERE user_id = ? AND client_token_id = ?",
        (user.id, &client_token.id),
    )? {
        None => {
            let temp_token_id = state.exec_sql_insert_id(
                "INSERT INTO temp_token (client_token_id, user_id, token) VALUES (?, ?, SUBSTRING(MD5(RAND()), 1, 10))",
                (&client_token.id, user.id),
            )?;

            match state.first_sql("SELECT * FROM temp_token WHERE id = ?", (&temp_token_id,))? {
                None => return Err(ErrorInternalServerError("could not find created auth_code")),
                Some(auth) => auth,
            }
        }
        Some(auth) => auth,
    };

    let body_str = state
        .templates
        .render(
            "first_flow",
            &json!({"link": "./second_flow?token=".to_string() + &temp_token.token}),
        )
        .expect("could not render data");
    Ok(HttpResponse::Ok().body(body_str))
}

pub fn second_flow(
    state: Data<AppState>,
    query: Query<SecondFlowQuery>,
) -> Result<HttpResponse, Error> {
    // TODO トランザクション
    let auth_code: TempToken = match get_temp_token(&state, query.token.to_string()) {
        Err(v) => return Err(v),
        Ok(v) => v,
    };

    state.exec_sql(
        "DELETE FROM temp_token WHERE token = ?",
        (&auth_code.token,),
    )?;

    let access_token_id = state.exec_sql_insert_id(
        "INSERT INTO access_token (token, refresh_token) VALUES(SUBSTRING(MD5(RAND()), 1, 10), SUBSTRING(MD5(RAND()), 1, 10))",(),
    )?;

    let access_token: AccessToken = match state.first_sql(
        "SELECT * FROM access_token WHERE id = ?",
        (&access_token_id,),
    )? {
        None => {
            return Err(ErrorInternalServerError(
                "could not find created access_token",
            ))
        }
        Some(auth) => auth,
    };

    // TODO WebHook

    Ok(success(
        200,
        json!({
            "access_token": access_token.token,
            "refresh_token": access_token.refresh_token,
        }),
    ))
}
