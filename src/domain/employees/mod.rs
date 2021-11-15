use anyhow::Result;

use super::{contract, value_objects::cpf::Cpf};
pub mod dto;

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

pub struct Shift {
  pub id: i32,
  pub name: String,
}

pub struct Role {
  pub id: i32,
  pub name: String,
}

pub struct Employee {
  pub id: i32,
  pub name: String,
  pub cpf: Cpf,
  pub wage_in_cents: i32,
  pub shift: Shift,
  pub role: Role,
  pub address: Address,
}

pub async fn get_all(db: &contract::Database) -> Result<Vec<Employee>> {
  db.employee.get_all().await
}

pub async fn create(db: &contract::Database, data: dto::Create) -> Result<Employee> {
  db.employee.create(data).await
}

pub async fn update(
  db: &contract::Database,
  employee_id: i32,
  data: dto::Update,
) -> Result<Employee> {
  db.employee.update(employee_id, data).await
}
