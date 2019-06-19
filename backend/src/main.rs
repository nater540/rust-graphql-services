#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate juniper;

use std::io;
use std::sync::Arc;

mod app;
mod db;

use crate::app::graphql::{Schema, create_schema};

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use juniper::http::GraphQLRequest;

fn graphql(st: web::Data<Arc<Schema>>, data: web::Json<GraphQLRequest>) -> impl Future<Item = HttpResponse, Error = Error> {
  web::block(move || {
    let res = data.execute(&st, &());
    Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
  })
  .map_err(Error::from)
  .and_then(|user| {
    Ok(HttpResponse::Ok()
      .content_type("application/json")
      .body(user)
    )
  })
}

fn main() -> io::Result<()> {
  let schema = std::sync::Arc::new(create_schema());

  // Start http server
  HttpServer::new(move || {
    App::new()
      .data(schema.clone())
      .wrap(middleware::Logger::default())
      .service(web::resource("/graphql").route(web::post().to_async(graphql)))
  })
  .bind("127.0.0.1:9000")?
  .run()
}
