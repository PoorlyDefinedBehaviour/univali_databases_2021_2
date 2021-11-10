use crate::domain::{condominiums, condominiums::contract};
use crate::routes::viewmodel;
use actix_web::{post, web, HttpResponse, Responder};

pub fn init(config: &mut web::ServiceConfig) {
  config.service(create);
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
