pub mod dto;

use anyhow::Result;

use super::{cities::City, contract, value_objects::cnpj::Cnpj};

#[derive(Debug)]
pub struct Address {
  pub id: i32,
  pub street: String,
  pub number: String,
  pub city: City,
}

#[derive(Debug)]
pub struct Condominium {
  pub id: i32,
  pub name: String,
  pub cnpj: Cnpj,
  pub address: Address,
}

pub async fn get_all(db: &contract::Database) -> Result<Vec<Condominium>> {
  db.condominium.get_all().await
}

pub async fn create(db: &contract::Database, data: dto::Create) -> Result<Condominium> {
  db.condominium.create(data).await
}

pub async fn update(
  db: &contract::Database,
  condominium_id: i32,
  data: dto::Update,
) -> Result<Condominium> {
  db.condominium.update(condominium_id, data).await
}

pub async fn delete(db: &contract::Database, condominium_id: i32) -> Result<()> {
  db.condominium.delete(condominium_id).await
}
