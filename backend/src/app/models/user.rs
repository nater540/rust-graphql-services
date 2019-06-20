// use diesel::prelude::*;
use argonautica::{Hasher};
use chrono::prelude::*;
use diesel::pg::PgConnection;
use serde::{Serialize};

use crate::db::users;

#[derive(Serialize, Queryable, PartialEq, Debug)]
pub struct User {
  pub id: i32,
  pub uuid: String,
  pub email: String,
  pub password_digest: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
  pub email: String,
  pub password_digest: String
}

impl User {
  pub fn update(self, _connection: &PgConnection) -> Result<(), failure::Error> {
    Ok(())
  }
}

impl NewUser {
  pub fn create<S>(email: S, password: S, connection: &PgConnection) -> Result<User, failure::Error> 
    where S: Into<String> {
    use diesel::RunQueryDsl;

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
      .with_secret_key(secret()?)
      .with_password(password)
      .hash()?)
  }
}

/// Gets the password secret hash from an environment variable.
fn secret() -> Result<String, failure::Error> {
  match std::env::var("HEIMDALLR_SECRET") {
    Ok(val)  => Ok(val),
    Err(_err) => panic!("TODO")
  }
}
