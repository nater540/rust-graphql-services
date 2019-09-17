
use juniper::{Context, FieldResult, RootNode};

use super::types::User;
use crate::db::Database;
use crate::app::models::User as DBUser;

// Allows you to use the database pool in GraphQL resolvers.
impl Context for Database {}

pub struct Query;
pub struct Mutation;

pub type Schema = RootNode<'static, Query, Mutation>;

#[juniper::object(Context = Database)]
impl Query {
  fn getUser(db: &Database, uuid: uuid::Uuid) -> FieldResult<User> {
    debug!("########## getUser {}", uuid);
    let user = DBUser::by_uuid(&uuid, &*db.pool.get()?)?;
    debug!("{:?}", user);

    Ok(user.into())
  }
}

#[juniper::object(Context = Database)]
impl Mutation {

}

pub fn create_schema() -> Schema {
  Schema::new(Query {}, Mutation {})
}
