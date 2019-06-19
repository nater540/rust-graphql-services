use openssl::ssl::{SslMethod, SslAcceptor, SslAcceptorBuilder, SslFiletype};
use actix_web::{http::header, middleware::Logger, App, HttpServer, web};
use actix_cors::Cors;

use crate::APP_SETTINGS;
use crate::db::Database;

pub struct Server {
  pub sys: actix_rt::SystemRunner
}

impl Server {
  /// Creates a new instance of the HTTP server.
  pub fn new() -> Result<Self, failure::Error> {
    let sys = actix_rt::System::new("backend");
    let db  = Database::new()?;

    let server = HttpServer::new(move || {
      App::new()
        .data(db.clone())
        .wrap(Logger::default())
        .wrap(
          Cors::new()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600)
        )

    });

    if let (Some(private_key), Some(cert)) = (&APP_SETTINGS.inbound_listener.private_key, &APP_SETTINGS.inbound_listener.cert) {
      let builder = Self::build_tls(&private_key, &cert)?;
      server.bind_ssl(&APP_SETTINGS.inbound_listener.address, builder)?.start();
    }
    else {
      server.bind(&APP_SETTINGS.inbound_listener.address)?.start();
    }

    Ok(Server{ sys })
  }

  pub fn run(self) -> Result<(), failure::Error> {
    Ok(self.sys.run()?)
  }

  /// Creates an SSL Acceptor object.
  /// 
  /// # Arguments
  /// * `tls` - TLS configuration settings.
  fn build_tls(private_key: &str, cert: &str) -> Result<SslAcceptorBuilder, failure::Error> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file(private_key, SslFiletype::PEM)?;
    builder.set_certificate_chain_file(cert)?;
    Ok(builder)
  }
}
