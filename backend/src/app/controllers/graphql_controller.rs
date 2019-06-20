use actix_web::{Error, HttpResponse, web};
use juniper::http::GraphQLRequest;
use futures::Future;
use std::sync::Arc;

use crate::app::graphql::Schema;

pub fn handler(st: web::Data<Arc<Schema>>, data: web::Json<GraphQLRequest>) -> impl Future<Item = HttpResponse, Error = Error> {
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
