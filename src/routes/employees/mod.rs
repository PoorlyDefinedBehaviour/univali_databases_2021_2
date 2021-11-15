use std::convert::TryInto;

use actix_web::{post, web, HttpResponse, Responder};

use crate::domain::{contract, employees};
mod viewmodel;

pub fn init(config: &mut web::ServiceConfig) {
  config.service(create);
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
