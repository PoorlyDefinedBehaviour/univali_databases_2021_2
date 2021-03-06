#[macro_use]
extern crate log;

use crate::infra::repositories;
use actix_cors::Cors;
use actix_web::{middleware, web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

mod domain;
mod infra;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "databases_1_m3_1=trace");

  dotenv().ok();

  env_logger::init();

  let database_url = env::var("DATABASE_URL").unwrap();

  info!("connecting to database");

  let db_pool = MySqlPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
    .unwrap();

  let host = env::var("HOST").unwrap();
  let port = env::var("PORT").unwrap().parse::<u16>().unwrap();

  info!("starting server at {}:{}", &host, port);

  HttpServer::new(move || {
    App::new()
      .app_data(Data::new(repositories::new(db_pool.clone())))
      .wrap(middleware::Logger::default())
      .wrap(Cors::permissive())
      .configure(routes::init)
  })
  .bind((host, port))?
  .run()
  .await
}
