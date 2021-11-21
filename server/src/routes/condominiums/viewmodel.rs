use crate::domain::{
  condominiums,
  value_objects::{self, cnpj::Cnpj},
};
pub use crate::routes::viewmodel::ValidationError;
use serde::{Deserialize, Serialize};
use std::convert::{From, TryFrom, TryInto};

#[derive(Deserialize, Serialize)]
pub struct State {
  pub id: i32,
  pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct City {
  pub id: i32,
  pub name: String,
  pub state: State,
}

#[derive(Deserialize, Serialize)]
pub struct Address {
  pub id: i32,
  pub street: String,
  pub number: String,
  pub city: City,
}

#[derive(Deserialize, Serialize)]
pub struct Condominium {
  pub id: i32,
  pub name: String,
  pub cnpj: String,
  pub address: Address,
}

impl From<condominiums::Condominium> for Condominium {
  fn from(item: condominiums::Condominium) -> Self {
    Condominium {
      id: item.id,
      name: item.name,
      cnpj: item.cnpj.into_inner(),
      address: Address {
        id: item.address.id,
        street: item.address.street,
        number: item.address.number,
        city: City {
          id: item.address.city.id,
          name: item.address.city.name,
          state: State {
            id: item.address.city.state.id,
            name: item.address.city.state.name,
          },
        },
      },
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCondominiumAddress {
  pub street: String,
  pub number: String,
  pub city_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Create {
  pub name: String,
  pub cnpj: String,
  pub address: CreateCondominiumAddress,
}

impl TryInto<condominiums::dto::Create> for Create {
  type Error = value_objects::ValidationError;

  fn try_into(self) -> Result<condominiums::dto::Create, Self::Error> {
    Ok(condominiums::dto::Create {
      name: self.name,
      cnpj: Cnpj::try_from(self.cnpj)?,
      address: condominiums::dto::Address {
        street: self.address.street,
        number: self.address.number,
        city_id: self.address.city_id,
      },
    })
  }
}

pub type Update = Create;
