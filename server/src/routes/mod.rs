use actix_web::web;

mod cities;
mod condominiums;
mod employees;
mod viewmodel;

pub fn init(config: &mut web::ServiceConfig) {
  condominiums::init(config);
  employees::init(config);
  cities::init(config);
}
