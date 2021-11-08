pub mod contract;
pub mod dto;

use anyhow::Result;

pub struct State {
  pub id: i32,
  pub name: String,
}

pub struct City {
  pub id: i32,
  pub name: String,
  pub state: State,
}

pub struct Address {
  pub id: i32,
  pub street: String,
  pub number: String,
  pub city: City,
}

pub struct Condominium {
  pub id: i32,
  pub name: String,
  pub cnpj: String,
  pub address: Address,
}

pub async fn create(
  db: &contract::repositories::Database,
  data: dto::condominium::Create,
) -> Result<Condominium> {
  db.condominium.create(data).await
}
