mod viewmodel;
use actix_web::{get, web, HttpResponse, Responder};

use crate::domain::{cities, contract};

pub fn init(config: &mut web::ServiceConfig) {
  config.service(get_all_cities);
}

#[get("/cities")]
async fn get_all_cities(db: web::Data<contract::Database>) -> impl Responder {
  match cities::get_all(db.get_ref()).await {
    Err(err) => {
      error!("{}", err);
      HttpResponse::ServiceUnavailable().finish()
    }
    Ok(cities) => {
      let view = cities
        .into_iter()
        .map(viewmodel::City::from)
        .collect::<Vec<_>>();

      HttpResponse::Ok().json(view)
    }
  }
}
