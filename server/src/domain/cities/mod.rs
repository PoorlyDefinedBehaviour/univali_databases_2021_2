use anyhow::Result;

use super::contract;

#[derive(Debug)]
pub struct State {
  pub id: i32,
  pub name: String,
}

#[derive(Debug)]
pub struct City {
  pub id: i32,
  pub name: String,
  pub state: State,
}

pub async fn get_all(db: &contract::Database) -> Result<Vec<City>> {
  db.city.get_all().await
}
