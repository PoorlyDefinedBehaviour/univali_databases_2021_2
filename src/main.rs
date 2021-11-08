#[macro_use]
extern crate log;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

mod domain;
mod infra;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  env_logger::init();

  let database_url = env::var("DATABASE_URL").unwrap();

  info!("connecting to database at {}", &database_url);

  let db_pool = MySqlPoolOptions::new()
    .connect(&database_url)
    .await
    .unwrap();

  let host = env::var("HOST").unwrap();
  let port = env::var("PORT").unwrap().parse::<u16>().unwrap();

  info!("starting server at {}:{}", &host, port);

  HttpServer::new(move || {
    App::new()
      .data(db_pool.clone())
      .wrap(middleware::Logger::default())
      .configure(routes::condominium::init)
  })
  .bind((host, port))?
  .run()
  .await
}
