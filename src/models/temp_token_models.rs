use chrono::NaiveDateTime;
use mysql;
use mysql::prelude::*;
use serde::Deserialize;

#[derive(Debug)]
pub struct TempToken {
    pub id: i64,
    pub user_id: i64,
    pub client_token_id: i64,
    pub token: String,
    pub created_at: NaiveDateTime,
}

impl FromRow for TempToken {
    fn from_row(row: mysql::Row) -> Self {
        Self::from_row_opt(row).expect("failed to deserialize data")
    }

    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError> {
        FromRow::from_row_opt(row).map(|(id, user_id, client_token_id, token, created_at)| Self {
            id,
            user_id,
            client_token_id,
            token,
            created_at,
        })
    }
}
