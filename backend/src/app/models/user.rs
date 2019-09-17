use argonautica::{Hasher};
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::{pg::PgConnection, ExpressionMethods, RunQueryDsl, QueryDsl};
use serde::{Serialize};

use crate::db::users;

/// Default secret to use when the environment variable doesn't exist [much better than just using `panic!` :P]
static DEFAULT_SECRET_HASH: &str = "44f5af520ce6154f624c4e08e21d1c4c37e82612f74674ff4bd13ee11a9ef775";

#[derive(Serialize, Queryable, Identifiable, PartialEq, Debug)]
pub struct User {
  pub id: i32,
  pub uuid: uuid::Uuid,
  pub email: String,
  pub password_digest: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
}

impl User {
  pub fn by_uuid(uuid: &uuid::Uuid, connection: &PgConnection) -> QueryResult<User> {
    users::table.filter(users::uuid.eq(uuid)).limit(1).first(&*connection)
    // let query = users::table.filter(users::uuid.eq(uuid)).limit(1);
    // info!("########## QUERY = {}", diesel::debug_query(&query));
    // query.first(&*connection)
  }

  pub fn update(self, _connection: &PgConnection) -> Result<(), failure::Error> {
    Ok(())
  }
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
  pub email: String,
  pub password_digest: String
}

impl NewUser {
  pub fn create<S>(email: S, password: S, connection: &PgConnection) -> Result<User, failure::Error> 
    where S: Into<String> {
    Ok(diesel::insert_into(users::table)
      .values(NewUser{
        email: email.into(),
        password_digest: Self::hash_password(&password.into())?
      })
      .get_result(connection)?
    )
  }

  /// Hashes a password.
  /// 
  /// # Arguments
  /// * `password` - The password to hash.
  fn hash_password<'a>(password: &'a str) -> Result<String, failure::Error> {
    Ok(Hasher::default()
      .with_secret_key(get_secret()?)
      .with_password(password)
      .hash()?)
  }
}

/// Gets the password secret from an environment variable.
fn get_secret() -> Result<String, failure::Error> {
  match std::env::var("HEIMDALLR_SECRET") {
    Ok(val)  => Ok(val),
    Err(_) => {
      warn!("Missing ENV[\"HEIMDALLR_SECRET\"] - Using default secret hash `{}`", DEFAULT_SECRET_HASH);
      Ok(DEFAULT_SECRET_HASH.to_string())
    }
  }
}
