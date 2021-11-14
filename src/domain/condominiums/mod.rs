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

pub async fn get_all(db: &contract::repositories::Database) -> Result<Vec<Condominium>> {
  db.condominium.get_all().await
}

pub async fn create(
  db: &contract::repositories::Database,
  data: dto::condominium::Create,
) -> Result<Condominium> {
  db.condominium.create(data).await
}

pub async fn update(
  db: &contract::repositories::Database,
  condominium_id: i32,
  data: dto::condominium::Update,
) -> Result<Condominium> {
  db.condominium.update(condominium_id, data).await
}

pub async fn delete(db: &contract::repositories::Database, condominium_id: i32) -> Result<()> {
  db.condominium.delete(condominium_id).await
}
