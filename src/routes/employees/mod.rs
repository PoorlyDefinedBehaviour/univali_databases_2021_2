use std::convert::TryInto;

use actix_web::{get, post, web, HttpResponse, Responder};

use crate::domain::{contract, employees};
mod viewmodel;

pub fn init(config: &mut web::ServiceConfig) {
  config.service(create).service(get_all);
}

#[get("/employees")]
async fn get_all(db: web::Data<contract::Database>) -> impl Responder {
  match employees::get_all(db.get_ref()).await {
    Err(err) => {
      error!("{}", err);
      HttpResponse::ServiceUnavailable().finish()
    }
    Ok(employees) => {
      let view = employees
        .into_iter()
        .map(viewmodel::Employee::from)
        .collect::<Vec<_>>();

      HttpResponse::Ok().json(view)
    }
  }
}

#[post("/employees")]
async fn create(
  db: web::Data<contract::Database>,
  data: web::Json<viewmodel::Create>,
) -> impl Responder {
  let db = db.get_ref();

  match data.into_inner().try_into() {
    Err(validation_error) => {
      HttpResponse::BadRequest().json(viewmodel::ValidationError::from(validation_error))
    }
    Ok(dto) => match employees::create(db, dto).await {
      Err(err) => {
        error!("{}", err);
        HttpResponse::ServiceUnavailable().finish()
      }
      Ok(employee) => HttpResponse::Ok().json(viewmodel::Employee::from(employee)),
    },
  }
}
