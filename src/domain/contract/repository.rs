use crate::domain::{
  condominiums::{dto, Condominium},
  employees,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct Database {
  pub condominium: Box<dyn CondominiumRepository>,
  pub employee: Box<dyn EmployeeRepository>,
}

#[async_trait]
pub trait CondominiumRepository {
  async fn get_all(&self) -> Result<Vec<Condominium>>;

  async fn create(&self, data: dto::condominium::Create) -> Result<Condominium>;

  async fn update(
    &self,
    condominium_id: i32,
    data: dto::condominium::Update,
  ) -> Result<Condominium>;

  async fn delete(&self, condominium_id: i32) -> Result<()>;
}

#[async_trait]
pub trait EmployeeRepository {
  async fn get_all(&self) -> Result<Vec<employees::Employee>>;

  async fn create(&self, data: employees::dto::Create) -> Result<employees::Employee>;

  async fn update(&self, employee_id: i32, data: employees::dto::Update) -> Result<Condominium>;
}
