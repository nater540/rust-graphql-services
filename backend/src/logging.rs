
use crate::APP_ARGS;

/// Initializes the logging system.
pub fn init() -> Result<(), fern::InitError> {
  let mut base_config = fern::Dispatch::new();

  base_config = match APP_ARGS.verbosity {
    0 => {
      base_config
        .level(log::LevelFilter::Info)
        .level_for("tokio_reactor", log::LevelFilter::Warn)
        .level_for("actix_web::server::server", log::LevelFilter::Warn)
      }
    1 => {
      base_config
        .level(log::LevelFilter::Debug)
        .level_for("tokio_reactor", log::LevelFilter::Info)
        .level_for("actix_web::server::server", log::LevelFilter::Info)
    }
    2 => base_config.level(log::LevelFilter::Debug),
    _ => base_config.level(log::LevelFilter::Trace)
  };

  let file_config = fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!(
          "{}[{}][{}] {}",
          chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
          record.target(),
          record.level(),
          message
      ))
    })
    .chain(fern::log_file("output.log")?);

  let stdout_config = fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!(
        "[{}][{}][{}] {}",
        chrono::Local::now().format("%H:%M"),
        record.target(),
        record.level(),
        message
      ))
    })
    .chain(std::io::stdout());

  base_config.chain(file_config).chain(stdout_config).apply()?;
  Ok(())
}
