pub mod viewmodel;
use std::convert::TryInto;

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

use crate::domain::{condominiums, contract};

pub fn init(config: &mut web::ServiceConfig) {
  config
    .service(get_all)
    .service(create)
    .service(update)
    .service(delete);
}

#[get("/condominiums")]
async fn get_all(db: web::Data<contract::Database>) -> impl Responder {
  let db = db.get_ref();

  match condominiums::get_all(db).await {
    Err(err) => {
      error!("{}", err);
      HttpResponse::ServiceUnavailable().finish()
    }
    Ok(condominiums) => {
      let view = condominiums
        .into_iter()
        .map(viewmodel::Condominium::from)
        .collect::<Vec<_>>();

      HttpResponse::Ok().json(view)
    }
  }
}

#[post("/condominiums")]
async fn create(
  db: web::Data<contract::Database>,
  data: web::Json<viewmodel::Create>,
) -> impl Responder {
  let db = db.get_ref();

  match data.into_inner().try_into() {
    Err(validation_error) => {
      HttpResponse::BadRequest().json(viewmodel::ValidationError::from(validation_error))
    }
    Ok(data) => match condominiums::create(db, data).await {
      Err(err) => {
        error!("{}", err);
        HttpResponse::ServiceUnavailable().finish()
      }
      Ok(condominium) => HttpResponse::Ok().json(viewmodel::Condominium::from(condominium)),
    },
  }
}

#[patch("/condominiums/{condominium_id}")]
async fn update(
  db: web::Data<contract::Database>,
  path_params: web::Path<i32>,
  data: web::Json<viewmodel::Update>,
) -> impl Responder {
  match data.into_inner().try_into() {
    Err(validation_error) => {
      HttpResponse::BadRequest().json(viewmodel::ValidationError::from(validation_error))
    }
    Ok(data) => match condominiums::update(db.get_ref(), path_params.into_inner(), data).await {
      Err(err) => {
        error!("{}", err);
        HttpResponse::ServiceUnavailable().finish()
      }
      Ok(condominium) => HttpResponse::Ok().json(viewmodel::Condominium::from(condominium)),
    },
  }
}

#[delete("/condominiums/{condominium_id}")]
async fn delete(db: web::Data<contract::Database>, path_params: web::Path<i32>) -> impl Responder {
  let db = db.get_ref();

  match condominiums::delete(db, path_params.into_inner()).await {
    Err(err) => {
      error!("{}", err);
      HttpResponse::ServiceUnavailable()
    }
    Ok(()) => HttpResponse::Ok(),
  }
}
