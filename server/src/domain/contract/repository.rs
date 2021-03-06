use crate::domain::{cities, condominiums, employees};
use anyhow::Result;
use async_trait::async_trait;

pub struct Database {
  pub condominium: Box<dyn CondominiumRepository>,
  pub employee: Box<dyn EmployeeRepository>,
  pub city: Box<dyn CityRepository>,
}

#[async_trait]
pub trait CondominiumRepository {
  async fn get_all(&self) -> Result<Vec<condominiums::Condominium>>;

  async fn create(&self, data: condominiums::dto::Create) -> Result<condominiums::Condominium>;

  async fn update(
    &self,
    condominium_id: i32,
    data: condominiums::dto::Update,
  ) -> Result<condominiums::Condominium>;

  async fn delete(&self, condominium_id: i32) -> Result<()>;
}

#[async_trait]
pub trait EmployeeRepository {
  async fn get_all(&self) -> Result<Vec<employees::Employee>>;

  async fn create(&self, data: employees::dto::Create) -> Result<employees::Employee>;

  async fn update(
    &self,
    employee_id: i32,
    data: employees::dto::Update,
  ) -> Result<employees::Employee>;

  async fn delete(&self, employee_id: i32) -> Result<()>;

  async fn get_all_shifts(&self) -> Result<Vec<employees::Shift>>;

  async fn get_all_roles(&self) -> Result<Vec<employees::Role>>;
}

#[async_trait]
pub trait CityRepository {
  async fn get_all(&self) -> Result<Vec<cities::City>>;
}
