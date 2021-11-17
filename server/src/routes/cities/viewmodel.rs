use serde::{Deserialize, Serialize};

use crate::domain::cities;

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

impl From<cities::City> for City {
  fn from(item: cities::City) -> Self {
    Self {
      id: item.id,
      name: item.name,
      state: State {
        id: item.state.id,
        name: item.state.name,
      },
    }
  }
}
