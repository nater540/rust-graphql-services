
use juniper::FieldResult;
use juniper::RootNode;

pub struct QueryRoot;
pub struct MutationRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

graphql_object!(QueryRoot: () |&self| {
  field human(&executor, id: String) -> FieldResult<Human> {
    Ok(Human{
      id: "1234".to_owned(),
      name: "Luke".to_owned(),
      appears_in: vec![Episode::NewHope],
      home_planet: "Mars".to_owned()
    })
  }
});
