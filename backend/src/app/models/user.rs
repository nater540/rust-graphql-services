use diesel::prelude::*;
use chrono::prelude::*;
use argonautica::{Hasher, Verifier};
use serde::{Serialize, Deserialize};

use crate::db::users;

#[derive(Serialize, Queryable, Debug)]
pub struct User {
  pub id: i32,
  pub uuid: uuid::Uuid,
  email: String,
  password_digest: String,
  created_at: NaiveDateTime,
  updated_at: NaiveDateTime
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub email: &'a str,
  pub password_digest: &'a str
}

impl<'a> NewUser<'a> {
  pub fn create(email: &'a str, password: &'a str) -> Result<Self, failure::Error> {
    Ok(NewUser{
      email: email,
      password_digest: &hash(password)?
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
fn hash<S>(password: S) -> Result<String, failure::Error>
  where S: Into<String> {
  Ok(Hasher::default()
    .with_secret_key(secret()?)
    .with_password(password.into())
    .hash()?)
}

// argonautica::error::Error
