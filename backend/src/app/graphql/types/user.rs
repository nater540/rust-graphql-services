use crate::app::models::User as DBUser;

#[derive(GraphQLObject)]
#[graphql(description = "User object")]
pub struct User {
  pub id: i32,
  pub uuid: uuid::Uuid,
  pub email: String,
  pub created_at: String,
  pub updated_at: String
}

impl From<DBUser> for User {
  fn from(db_user: DBUser) -> Self {
    User{
      id: db_user.id.clone(),
      uuid: db_user.uuid.clone(),
      email: db_user.email.clone(),
      created_at: chrono::Utc::now().to_rfc3339(), // TODO: Convert NaiveDateTime
      updated_at: chrono::Utc::now().to_rfc3339()  // TODO: Convert NaiveDateTime
    }
  }
}
