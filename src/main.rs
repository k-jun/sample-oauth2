use actix_cors::Cors;
use actix_web::{
    error::Error,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware::Logger,
    web::Data,
    App, HttpServer,
};
use dotenv::dotenv;
use std::env;
mod routers;
use routers::routes;
extern crate log;
extern crate mysql;
use mysql::prelude::*;
mod controllers;
mod entities;
mod models;
use failure::Error as FailureError;
use handlebars::Handlebars;
use std::sync::Arc;

use actix_session::CookieSession;

fn err_handle(e: impl ::failure::Fail) -> Error {
    let e: FailureError = e.into();
    e.into()
}

#[derive(Clone)]
pub struct AppState {
    pub db: mysql::Pool,
    pub templates: Arc<Handlebars>,
}

impl AppState {
    fn exec_sql(&self, sql: impl AsRef<str>, param: impl Into<mysql::Params>) -> Result<(), Error> {
        self.db
            .prep_exec(sql.as_ref(), param)
            .map(|_| ())
            .map_err(err_handle)
    }

    fn exec_sql_insert_id(
        &self,
        sql: impl AsRef<str>,
        param: impl Into<mysql::Params>,
    ) -> Result<u64, Error> {
        self.db
            .prep_exec(sql.as_ref(), param)
            .map(|result| result.last_insert_id())
            .map_err(err_handle)
    }

    fn query_sql<T: FromRow>(
        &self,
        sql: impl AsRef<str>,
        param: impl Into<mysql::Params>,
    ) -> Result<Vec<T>, Error> {
        self.db
            .prep_exec(sql, param)
            .map_err(err_handle)?
            .map(|ret| ret.map(T::from_row))
            .collect::<Result<Vec<T>, _>>()
            .map_err(err_handle)
    }

    fn first_sql<T: FromRow>(
        &self,
        sql: impl AsRef<str>,
        param: impl Into<mysql::Params>,
    ) -> Result<Option<T>, Error> {
        self.db
            .first_exec(sql, param)
            .map_err(err_handle)
            .map(|opt| opt.map(T::from_row))
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let db_url = env::var("DB_ADDRESS").expect("could not find DB_ADDRESS");
    let client_domain = env::var("CLIENT_DOMAIN").expect("could not find CLIENT_DOMAIN");
    let bind_address = env::var("BIND_ADDRESS").expect("could not find BIND_ADDRESS");

    let mut templates = Handlebars::new();
    templates
        .register_template_string("first_flow", include_str!("./views/first_flow.html"))
        .expect("failed to register template");

    let state = AppState {
        db: mysql::Pool::new(db_url).expect("could not connect to db"),
        templates: Arc::new(templates),
    };
    let server = HttpServer::new(move || {
        App::new()
            .register_data(Data::new(state.clone()))
            .wrap(
                Cors::new()
                    .allowed_origin(&client_domain)
                    .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                    .max_age(3600),
            )
            .wrap(
                CookieSession::signed(&[0; 32]) // <- create cookie based session middleware
                    .secure(false),
            )
            .wrap(Logger::default())
            .configure(routes)
    });

    server
        .bind(bind_address)
        .expect("could not bind address")
        .run()
        .expect("could not run server");
}
