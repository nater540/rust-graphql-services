#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_json;

mod app;
mod db;

use app::models::NewUser;

fn main() {
  let new_user = NewUser::create("nater540@gmail.com", "supercalifragilisticexpialidocious");
  println!("{:?}", new_user);
}
