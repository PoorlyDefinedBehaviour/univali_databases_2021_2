use std::convert::TryInto;

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

use crate::domain::{contract, employees};
mod viewmodel;

pub fn init(config: &mut web::ServiceConfig) {
  config
    .service(get_all)
    .service(create)
    .service(update)
    .service(delete);
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
  match data.into_inner().try_into() {
    Err(validation_error) => {
      HttpResponse::BadRequest().json(viewmodel::ValidationError::from(validation_error))
    }
    Ok(dto) => match employees::create(db.get_ref(), dto).await {
      Err(err) => {
        error!("{}", err);
        HttpResponse::ServiceUnavailable().finish()
      }
      Ok(employee) => HttpResponse::Ok().json(viewmodel::Employee::from(employee)),
    },
  }
}

#[patch("/employees/{employee_id}")]
async fn update(
  db: web::Data<contract::Database>,
  employee_id: web::Path<i32>,
  data: web::Json<viewmodel::Update>,
) -> impl Responder {
  match data.into_inner().try_into() {
    Err(validation_error) => {
      HttpResponse::BadRequest().json(viewmodel::ValidationError::from(validation_error))
    }
    Ok(dto) => match employees::update(db.get_ref(), employee_id.into_inner(), dto).await {
      Err(err) => {
        error!("{}", err);
        HttpResponse::ServiceUnavailable().finish()
      }
      Ok(employee) => HttpResponse::Ok().json(viewmodel::Employee::from(employee)),
    },
  }
}

#[delete("/employees/{employee_id}")]
async fn delete(db: web::Data<contract::Database>, employee_id: web::Path<i32>) -> impl Responder {
  match employees::delete(db.get_ref(), employee_id.into_inner()).await {
    Err(err) => {
      error!("{}", err);
      HttpResponse::ServiceUnavailable()
    }
    Ok(()) => HttpResponse::Ok(),
  }
}
