use crate::domain::value_objects::cpf::Cpf;

pub struct Address {
  pub street: String,
  pub number: String,
  pub city_id: i32,
}

pub struct Create {
  pub name: String,
  pub cpf: Cpf,
  pub wage_in_cents: u32,
  pub works_at_condominium_id: i32,
  pub shift_id: i32,
  pub role_id: i32,
  pub address: Address,
}

pub type Update = Create;
