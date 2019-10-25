use chrono::NaiveDateTime;
use mysql;
use mysql::prelude::*;
use serde::Deserialize;

#[derive(Debug)]
pub struct AccessToken {
    pub id: i64,
    pub token: String,
    pub refresh_token: String,
    pub created_at: NaiveDateTime,
}

impl FromRow for AccessToken {
    fn from_row(row: mysql::Row) -> Self {
        Self::from_row_opt(row).expect("failed to deserialize data")
    }

    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError> {
        FromRow::from_row_opt(row).map(|(id, token, refresh_token, created_at)| Self {
            id,
            token,
            refresh_token,
            created_at,
        })
    }
}
