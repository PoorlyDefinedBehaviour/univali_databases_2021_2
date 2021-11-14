use crate::domain::{condominiums, condominiums::contract};
use crate::routes::viewmodel;
use actix_web::{get, patch, post, web, HttpResponse, Responder};

pub fn init(config: &mut web::ServiceConfig) {
  config.service(get_all).service(create).service(update);
}

#[get("/condominiums")]
async fn get_all(db: web::Data<contract::repositories::Database>) -> impl Responder {
  let db = db.get_ref();

  match condominiums::get_all(&db).await {
    Err(err) => {
      error!("{}", err);
      HttpResponse::ServiceUnavailable().finish()
    }
    Ok(condominiums) => {
      let view = condominiums
        .into_iter()
        .map(viewmodel::condominium::Condominium::from)
        .collect::<Vec<_>>();

      HttpResponse::Ok().json(view)
    }
  }
}

#[post("/condominiums")]
async fn create(
  db: web::Data<contract::repositories::Database>,
  data: web::Json<viewmodel::condominium::Create>,
) -> impl Responder {
  let db = db.get_ref();

  match condominiums::create(&db, data.into_inner().into()).await {
    Err(err) => {
      error!("{}", err);
      HttpResponse::ServiceUnavailable().finish()
    }
    Ok(condominium) => {
      HttpResponse::Ok().json(viewmodel::condominium::Condominium::from(condominium))
    }
  }
}

#[patch("/condominiums/{condominium_id}")]
async fn update(
  db: web::Data<contract::repositories::Database>,
  path_params: web::Path<i32>,
  data: web::Json<viewmodel::condominium::Update>,
) -> impl Responder {
  let db = db.get_ref();

  match condominiums::update(&db, path_params.into_inner(), data.into_inner().into()).await {
    Err(err) => {
      error!("{}", err);
      HttpResponse::ServiceUnavailable().finish()
    }
    Ok(condominium) => {
      HttpResponse::Ok().json(viewmodel::condominium::Condominium::from(condominium))
    }
  }
}
