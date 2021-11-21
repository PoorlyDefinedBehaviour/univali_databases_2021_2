use crate::domain::value_objects::cnpj::Cnpj;

pub struct Address {
  pub street: String,
  pub number: String,
  pub city_id: i32,
}

pub struct Create {
  pub name: String,
  pub cnpj: Cnpj,
  pub address: Address,
}

pub type Update = Create;
