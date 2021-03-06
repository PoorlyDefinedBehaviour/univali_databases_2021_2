use anyhow::Result;

use super::{cities::City, condominiums::Condominium, contract, value_objects::cpf::Cpf};
pub mod dto;

#[derive(Debug)]
pub struct Address {
  pub id: i32,
  pub street: String,
  pub number: String,
  pub city: City,
}

#[derive(Debug)]
pub struct Shift {
  pub id: i32,
  pub name: String,
}

#[derive(Debug)]
pub struct Role {
  pub id: i32,
  pub name: String,
}

#[derive(Debug)]
pub struct Employee {
  pub id: i32,
  pub name: String,
  pub cpf: Cpf,
  pub wage_in_cents: i32,
  pub shift: Shift,
  pub role: Role,
  pub address: Address,
  pub works_at_condominium: Condominium,
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

pub async fn delete(db: &contract::Database, employee_id: i32) -> Result<()> {
  db.employee.delete(employee_id).await
}

pub async fn get_all_shifts(db: &contract::Database) -> Result<Vec<Shift>> {
  db.employee.get_all_shifts().await
}

pub async fn get_all_roles(db: &contract::Database) -> Result<Vec<Role>> {
  db.employee.get_all_roles().await
}
