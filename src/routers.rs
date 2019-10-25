use crate::controllers::*;
use actix_web::web;

pub fn routes(app: &mut web::ServiceConfig) {
  app
    .service(web::resource("/").to(index_controllers::index))
    .service(web::resource("/login").to(user_controllers::login))
    .service(web::resource("/logout").to(user_controllers::logout))
    .service(web::resource("/first_flow").to(auth_controllers::first_flow))
    .service(web::resource("/second_flow").to(auth_controllers::second_flow))
    .service(
      web::scope("/params")
        .route(
          "path/{f_str}/{f_int}/{f_flt}",
          web::get().to(index_controllers::path),
        )
        .route("query", web::get().to(index_controllers::query))
        .route("body", web::post().to(index_controllers::body)),
    )
    .service(
      web::scope("/user")
        .service(
          web::resource("")
            .route(web::get().to(user_controllers::index))
            .route(web::post().to(user_controllers::create)),
        )
        .service(
          web::resource("/{id}")
            .route(web::get().to(user_controllers::read))
            .route(web::put().to(user_controllers::update))
            .route(web::delete().to(user_controllers::delete)),
        ),
    )
    .service(
      web::scope("/client_token")
        .service(
          web::resource("")
            .route(web::get().to(client_token_controllers::index))
            .route(web::post().to(client_token_controllers::create)),
        )
        .service(
          web::resource("/{id}")
            .route(web::get().to(client_token_controllers::read))
            .route(web::delete().to(client_token_controllers::delete)),
        ),
    );
}
