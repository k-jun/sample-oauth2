use chrono::NaiveDateTime;
use mysql;
use mysql::prelude::*;
use serde::Deserialize;

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub authority_level: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl FromRow for User {
    fn from_row(row: mysql::Row) -> Self {
        Self::from_row_opt(row).expect("failed to deserialize data")
    }

    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError> {
        FromRow::from_row_opt(row).map(
            |(id, email, password, authority_level, created_at, updated_at)| Self {
                id,
                email,
                password,
                authority_level,
                created_at,
                updated_at,
            },
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct UserPath {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct UserNew {
    pub email: String,
    pub password: String,
    pub authority_level: i64,
}

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}
