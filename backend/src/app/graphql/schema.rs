
use juniper::{Context, FieldResult, RootNode};

use super::types::User;
use crate::db::Database;

// Allows you to use the database pool in GraphQL resolvers.
impl Context for Database {}

pub struct Query;
pub struct Mutation;

pub type Schema = RootNode<'static, Query, Mutation>;

#[juniper::object(Context = Database)]
impl Query {
  fn getUser(context: &Database, uuid: String) -> FieldResult<User> {
    
    Ok(User{
      id: 42,
      uuid: "".to_owned(),
      email: "nater540@gmail.com".to_owned(),
      created_at: chrono::Utc::now().to_rfc3339(),
      updated_at: chrono::Utc::now().to_rfc3339()
    })
  }
}

#[juniper::object(Context = Database)]
impl Mutation {

}

pub fn create_schema() -> Schema {
  Schema::new(Query {}, Mutation {})
}
