mod schema;
pub use schema::*;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::APP_SETTINGS;

#[derive(Clone)]
pub struct Database {
  pub pool: Pool<ConnectionManager<PgConnection>>
}

impl Database {
  pub fn new() -> Result<Self, failure::Error> {
    // Format a postgres connection string
    let database_url = format!(
      "postgres://{}:{}@{}:{}/{}",
      APP_SETTINGS.database.username.to_owned(),
      APP_SETTINGS.database.password.to_owned(),
      APP_SETTINGS.database.host.to_owned(),
      APP_SETTINGS.database.port.unwrap_or(5432),
      APP_SETTINGS.database.name.to_owned()
    );

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool    = Pool::builder().build(manager)?;
    Ok(Database { pool })
  }
}
