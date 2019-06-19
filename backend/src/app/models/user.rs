use diesel::prelude::*;
use chrono::prelude::*;
use argonautica::{Hasher, Verifier};
use serde::{Serialize, Deserialize};

use crate::db::users;

#[derive(Serialize, Queryable, Debug)]
pub struct User {
  pub id: i32,
  pub uuid: uuid::Uuid,
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

impl NewUser {
  pub fn create<S>(email: S, password: S) -> Result<Self, failure::Error> 
    where S: Into<String> {
    Ok(NewUser{
      email: email.into(),
      password_digest: hash(&password.into())?
    })
  }
}

/// Gets the password secret hash from an environment variable.
fn secret() -> Result<String, failure::Error> {
  match std::env::var("HEIMDALLR_SECRET") {
    Ok(val)  => Ok(val),
    Err(err) => panic!("TODO")
  }
}

/// Hashes a password.
/// 
/// # Arguments
/// * `password` - The password to hash.
fn hash<'a>(password: &'a str) -> Result<String, failure::Error> {
  Ok(Hasher::default()
    .with_secret_key(secret()?)
    .with_password(password)
    .hash()?)
}

// argonautica::error::Error
