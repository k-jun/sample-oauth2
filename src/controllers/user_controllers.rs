use super::super::AppState;
use crate::models::user_models::*;
use actix_web::{
    error::Error,
    error::ErrorNotFound,
    web::{Data, Json, Path},
    HttpResponse,
};
#[macro_use]
use serde_json::json;
use crate::controllers::common::*;
use actix_session::Session;

pub fn login(
    (state, json, session): (Data<AppState>, Json<UserLogin>, Session),
) -> Result<HttpResponse, Error> {
    let user: User = match state.first_sql("SELECT * FROM user WHERE email = ?", (&json.email,))? {
        None => return Ok(failure(404)),
        Some(user) => user,
    };

    // set session
    session.set(SESSION_NAME, user.id)?;
    Ok(success(200, json!({})))
}

pub fn logout(session: Session) -> Result<HttpResponse, Error> {
    // clear session
    session.set(SESSION_NAME, -1)?;
    Ok(success(200, json!({})))
}

fn get_user(state: &Data<AppState>, user_id: String) -> Result<User, Error> {
    match state.first_sql("SELECT * FROM user WHERE id = ?", (user_id,))? {
        None => return Err(ErrorNotFound("could not find user")),
        Some(u) => Ok(u),
    }
}

pub fn index(state: Data<AppState>, sess: Session) -> Result<HttpResponse, Error> {
    let user = match get_sessuser(&state, sess) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };
    if user.authority_level < 900 {
        return Ok(failure(403));
    }

    let users: Vec<User> = state.query_sql("SELECT * FROM user", ())?;
    Ok(success(
        200,
        json!({ "users": users
            .iter()
            .map(|user| {
                json!({
                    "id": user.id,
                    "email": user.email,
                    "created_at": user.created_at.to_string(),
                    "updated_at": user.updated_at.to_string(),
                })
            }).collect::<Vec<mysql::serde_json::Value>>() }),
    ))
}

pub fn read(
    state: Data<AppState>,
    path: Path<UserPath>,
    sess: Session,
) -> Result<HttpResponse, Error> {
    let user = match get_sessuser(&state, sess) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };
    if user.authority_level < 500 {
        return Ok(failure(403));
    }

    let user: User = match state.first_sql("SELECT * FROM user WHERE id = ?", (&path.id,))? {
        None => return Ok(failure(404)),
        Some(v) => v,
    };

    Ok(success(
        200,
        json!({
          "id": user.id,
          "email": user.email,
          "created_at": user.created_at.to_string(),
          "updated_at": user.updated_at.to_string(),
        }),
    ))
}

pub fn create(
    state: Data<AppState>,
    json: Json<UserNew>,
    sess: Session,
) -> Result<HttpResponse, Error> {
    // let user = match get_sessuser(&state, sess) {
    //     Ok(user) => user,
    //     Err(e) => return Err(e),
    // };
    // if user.authority_level < 900 {
    //     return Ok(failure(403));
    // }

    // if user.authority_level < json.authority_level {
    //     return Ok(failure(403));
    // }

    state.exec_sql(
        "INSERT INTO user (email, password, authority_level) VALUES (?, ?, ?)",
        (&json.email, &json.password, &json.authority_level),
    )?;

    Ok(success(200, json!("")))
}

pub fn update(
    state: Data<AppState>,
    path: Path<UserPath>,
    json: Json<UserNew>,
    sess: Session,
) -> Result<HttpResponse, Error> {
    let user = match get_sessuser(&state, sess) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };
    if user.authority_level < 900 {
        return Ok(failure(403));
    }

    let t_user = match get_user(&state, path.id.to_string()) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };

    if user.authority_level < t_user.authority_level {
        return Ok(failure(403));
    }

    if user.authority_level < json.authority_level {
        return Ok(failure(403));
    }

    state.exec_sql(
        "UPDATE user SET email = ?, password = ?, authority_level = ? WHERE id = ?",
        (&json.email, &json.password, &json.authority_level, &path.id),
    )?;

    Ok(success(200, json!("")))
}

pub fn delete(
    state: Data<AppState>,
    path: Path<UserPath>,
    sess: Session,
) -> Result<HttpResponse, Error> {
    let user = match get_sessuser(&state, sess) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };
    if user.authority_level < 900 {
        return Ok(failure(403));
    }

    let t_user = match get_user(&state, path.id.to_string()) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };

    if user.authority_level < t_user.authority_level {
        return Ok(failure(403));
    }

    state.exec_sql("DELETE FROM user WHERE id = ?", (&path.id,))?;

    Ok(success(200, json!("")))
}
