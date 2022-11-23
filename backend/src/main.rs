use std::env;
// use std::fs::File;
// use std::io::BufReader;

use actix_cors::Cors;
use actix_web::{http::header, middleware, App, HttpServer};
// use rustls::internal::pemfile::{certs, pkcs8_private_keys};
// use rustls::{NoClientAuth, ProtocolVersion, ServerConfig};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use anyhow::Result;
use db::K_RSSMAILER_DB;
use sqlx::postgres::PgPoolOptions;

#[macro_use]
extern crate dotenv_codegen;

pub mod db;
pub mod handlers;
pub mod services;
use handlers::account::*;
use handlers::index::index;
use handlers::rssfeed::*;
use handlers::error::*;
use services::mailer::Mailer;
use actix_web::{http, web::Data};
use actix_web::middleware::{ErrorHandlers};

#[actix_web::main]
async fn run_http_service() {
    env::set_var("RSSMAILER_LOG", "info");
    env_logger::init();
    let db_url = K_RSSMAILER_DB;
    let result = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await;
    let csrf_token_header = header::HeaderName::from_lowercase(b"x-csrf-token").unwrap();

    // let mut config = ServerConfig::new(NoClientAuth::new());
    // let cert_file = &mut BufReader::new(File::open("127.0.0.1+1.pem").unwrap());
    // let key_file = &mut BufReader::new(File::open("127.0.0.1+1-key.pem").unwrap());
    // let cert_chain = certs(cert_file).unwrap();
    // let mut keys = pkcs8_private_keys(key_file).unwrap();
    // config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
    // let protos = vec!["h2".to_string().into(), "http/1.1".to_string().into()]; 
    // config.set_protocols(&protos); 
    // config.versions = vec![ProtocolVersion::TLSv1_1];
    let mut builder =
    SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("127.0.0.1+1-key.pem", SslFiletype::PEM)
    .unwrap();
    builder.set_certificate_chain_file("127.0.0.1+1.pem").unwrap();

    match result {
        Ok(pool) => {
            let server = HttpServer::new(move || {
                let cros = Cors::default()
                    // .allowed_origin("127.0.0.1")
                    .allowed_methods(vec!["POST"])
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::CONTENT_TYPE,
                        header::ACCEPT,
                        csrf_token_header.clone(),
                    ])
                    .expose_headers(vec![csrf_token_header.clone()])
                    .max_age(3600);

                App::new()
                    .wrap(
                        ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500)
                    )
                    .app_data(Data::new(pool.clone()))
                    // .wrap(middleware::Compress::default())
                    // .wrap(middleware::Logger::default())
                    .wrap(cros)
                    .service(post_add_account)
                    .service(post_edit_account)
                    .service(post_delete_account)
                    .service(post_account)
                    .service(post_login_account)
                    .service(send_verify_email_code)
                    .service(verify_email)
                    .service(post_rssfeeds)
                    .service(index)
            });
            server
                // .bind_rustls("127.0.0.1:8443", config)
                // .bind_openssl('127.0.0.1:8443', builder)?
                .bind_openssl("127.0.0.1:8443", builder)
                .unwrap()
                .run()
                .await
                .unwrap();
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }
}

#[tokio::main]
async fn run_mailer() {
    let mailer = Mailer::new().await.unwrap();
    mailer.run().await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_service_handle = tokio::task::spawn_blocking(|| {
        run_http_service();
        println!("finish http service");
    });
    let mailer_handle = tokio::task::spawn_blocking(|| {
        run_mailer();
        println!("finish mailer");
    });
    mailer_handle.await.unwrap();
    http_service_handle.await.unwrap();
    Ok(())
}
