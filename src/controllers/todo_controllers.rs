use super::super::AppState;
use crate::models::todo_models::*;
use actix_web::{
  error::Error,
  web::{Data, Json, Path},
  HttpResponse,
};
#[macro_use]
use serde_json::json;

use crate::controllers::common::{failure, success};

fn exist_check(state: &Data<AppState>, id: String) -> Result<bool, Error> {
  let _: Todo = match state.first_sql("SELECT * FROM todo WHERE id = ?", (id,))? {
    None => return Ok(false),
    Some(todo) => todo,
  };
  return Ok(true);
}

pub fn index(state: Data<AppState>) -> Result<HttpResponse, Error> {
  let todos: Vec<Todo> = state.query_sql("SELECT * FROM todo", ())?;

  Ok(success(
    200,
    json!({ "todos": todos
    .iter()
    .map(|todo| {
      json!({
        "id": todo.id,
        "name": todo.name,
        "created_at": todo.created_at.to_string(),
        "updated_at": todo.updated_at.to_string(),
      })
    })
    .collect::<Vec<mysql::serde_json::Value>>() }),
  ))
}

pub fn read((state, path): (Data<AppState>, Path<TodoPath>)) -> Result<HttpResponse, Error> {
  let todo: Todo = match state.first_sql("SELECT * FROM todo WHERE id = ?", (&path.id,))? {
    None => return Ok(failure(403)),
    Some(todo) => todo,
  };

  Ok(success(
    200,
    json!({
      "id": todo.id,
      "name": todo.name,
      "created_at": todo.created_at.to_string(),
      "updated_at": todo.updated_at.to_string(),
    }),
  ))
}

pub fn create((state, json): (Data<AppState>, Json<TodoNew>)) -> Result<HttpResponse, Error> {
  state.exec_sql("INSERT INTO todo (name) VALUES (?)", (&json.name,))?;

  Ok(success(200, json!("")))
}

pub fn update(
  (state, json, path): (Data<AppState>, Json<TodoNew>, Path<TodoPath>),
) -> Result<HttpResponse, Error> {
  if !exist_check(&state, path.id.to_string())? {
    return Ok(failure(404));
  }

  state.exec_sql(
    "UPDATE todo SET name = ? WHERE id = ?",
    (&json.name, &path.id),
  )?;

  Ok(success(200, json!("")))
}

pub fn delete((state, path): (Data<AppState>, Path<TodoPath>)) -> Result<HttpResponse, Error> {
  if !exist_check(&state, path.id.to_string())? {
    return Ok(failure(404));
  }
  state.exec_sql("DELETE FROM todo WHERE id = ?", (&path.id,))?;

  Ok(success(200, json!("")))
}
