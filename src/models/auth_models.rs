use chrono::NaiveDateTime;
use mysql;
use mysql::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FirstFlowQuery {
    pub client_token: String,
    pub client_secret_token: String,
}

#[derive(Debug, Deserialize)]
pub struct SecondFlowQuery {
    pub token: String,
}
