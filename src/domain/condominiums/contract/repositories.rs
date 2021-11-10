use crate::domain::condominiums::{dto, Condominium};
use anyhow::Result;
use async_trait::async_trait;

pub struct Database {
  pub condominium: Box<dyn CondominiumRepository>,
}

#[async_trait]
pub trait CondominiumRepository {
  async fn create(&self, data: dto::condominium::Create) -> Result<Condominium>;

  async fn update(
    &self,
    condominium_id: i32,
    data: dto::condominium::Create,
  ) -> Result<Condominium>;
}
