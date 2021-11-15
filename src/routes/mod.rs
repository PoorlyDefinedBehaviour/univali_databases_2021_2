use actix_web::web;

mod condominiums;
mod employees;

pub fn init(config: &mut web::ServiceConfig) {
  condominiums::init(config);
  employees::init(config);
}
