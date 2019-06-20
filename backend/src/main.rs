#[macro_use]
extern crate clap;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate juniper;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

mod app;
mod db;
mod logging;
mod settings;
mod server;

use clap::{App, Arg};
use std::ops::Deref;

use crate::settings::Settings;

struct AppArgs {
  config: String,
  verbosity: u64
}

impl AppArgs {
  fn new() -> Self {
    let arguments = App::new("Backend")
    .about("Example Backend Service")
    .version(crate_version!())
    .arg(
      Arg::with_name("config")
        .long("config")
        .short("c")
        .value_name("FILE")
        .default_value("./default-config.yaml")
        .help("Sets a custom config file")
        .takes_value(true)
    )
    .arg(
      Arg::with_name("verbose")
        .long("verbose")
        .short("v")
        .multiple(true)
        .help("Increases logging verbosity each use for up to 3 times")
    ).get_matches();

    AppArgs {
      config:    String::from(arguments.value_of("config").expect("invalid config value")),
      verbosity: arguments.occurrences_of("verbose")
    }
  }
}

lazy_static! {
  static ref APP_ARGS: AppArgs      = AppArgs::new();
  static ref APP_SETTINGS: Settings = Settings::new();
}

fn main() {
  if let Err(ref _err) = run() {
    // use std::io::Write;

    // let stderr = &mut std::io::stderr();
    // let errmsg = "Error writing to stderr";

    // writeln!(stderr, "Encountered one or more errors:").expect(errmsg);
    // for err in err.iter().skip(1) {
    //   writeln!(stderr, "  - {}", err).expect(errmsg);
    // }

    // if let Some(backtrace) = err.backtrace() {
    //   writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
    // }

    std::process::exit(1);
  }
}

fn run() -> Result<(), failure::Error> {
  // Ensure all statics are valid
  let (_, _) = (APP_ARGS.deref(), APP_SETTINGS.deref());

  logging::init().expect("Failed to initialize logging.");

  let server = server::Server::new()?;
  server.run()?;
  Ok(())
}
