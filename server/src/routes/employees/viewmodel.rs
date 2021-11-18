use std::convert::{TryFrom, TryInto};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::domain::{employees, value_objects, value_objects::cpf::Cpf};

#[derive(Serialize, Deserialize)]
pub struct ValidationError {
  pub field: &'static str,
  pub message: String,
}

impl From<value_objects::ValidationError> for ValidationError {
  fn from(item: value_objects::ValidationError) -> Self {
    ValidationError {
      field: item.field,
      message: item.message,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct CreateEmployeeAddress {
  pub street: String,
  pub number: String,
  pub city_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Create {
  pub name: String,
  pub cpf: String,
  pub wage_in_cents: u32,
  pub works_at_condominium_id: i32,
  pub shift_id: i32,
  pub role_id: i32,
  pub address: CreateEmployeeAddress,
}

impl TryInto<employees::dto::Create> for Create {
  type Error = value_objects::ValidationError;

  fn try_into(self) -> Result<employees::dto::Create, Self::Error> {
    Ok(employees::dto::Create {
      name: self.name,
      cpf: Cpf::try_from(self.cpf)?,
      wage_in_cents: self.wage_in_cents,
      works_at_condominium_id: self.works_at_condominium_id,
      shift_id: self.shift_id,
      role_id: self.role_id,
      address: employees::dto::Address {
        street: self.address.street,
        number: self.address.number,
        city_id: self.address.city_id,
      },
    })
  }
}

#[derive(Serialize, Deserialize)]
pub struct State {
  pub id: i32,
  pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct City {
  pub id: i32,
  pub name: String,
  pub state: State,
}

#[derive(Serialize, Deserialize)]
pub struct Address {
  pub id: i32,
  pub street: String,
  pub number: String,
  pub city: City,
}

#[derive(Serialize, Deserialize)]
pub struct Employee {
  pub id: i32,
  pub name: String,
  pub cpf: String,
  pub wage_in_cents: i32,
  pub shift: String,
  pub role: String,
  pub address: Address,
}

impl From<employees::Employee> for Employee {
  fn from(item: employees::Employee) -> Self {
    Employee {
      id: item.id,
      name: item.name,
      cpf: item.cpf.into_inner(),
      wage_in_cents: item.wage_in_cents,
      shift: item.shift.name,
      role: item.role.name,
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

pub type Update = Create;

#[derive(Serialize, Deserialize)]
pub struct Shift {
  pub id: i32,
  pub name: String,
}

impl From<employees::Shift> for Shift {
  fn from(item: employees::Shift) -> Self {
    Self {
      id: item.id,
      name: item.name,
    }
  }
}
