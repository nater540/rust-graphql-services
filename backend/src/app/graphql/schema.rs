
use juniper::FieldResult;
use juniper::RootNode;

use super::types::User;

pub struct QueryRoot;
pub struct MutationRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

graphql_object!(QueryRoot: () |&self| {
  field getUser(&executor, id: String) -> FieldResult<User> {
    Ok(User{
      id: 42,
      uuid: "".to_owned(),
      email: "nater540@gmail.com".to_owned(),
      created_at: chrono::Utc::now().to_rfc3339(),
      updated_at: chrono::Utc::now().to_rfc3339()
    })
  }
});

graphql_object!(MutationRoot: () |&self| {
});

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}
