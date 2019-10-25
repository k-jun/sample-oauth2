use chrono::NaiveDateTime;
use mysql;
use mysql::prelude::*;
use serde::Deserialize;

#[derive(Debug)]
pub struct ClientToken {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub secret_token: String,
}

impl FromRow for ClientToken {
    fn from_row(row: mysql::Row) -> Self {
        Self::from_row_opt(row).expect("failed to deserialize data")
    }

    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError> {
        FromRow::from_row_opt(row).map(|(id, user_id, token, secret_token)| Self {
            id,
            user_id,
            token,
            secret_token,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct ClientTokenPath {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ClientTokenNew {
    pub user_id: i64,
}
