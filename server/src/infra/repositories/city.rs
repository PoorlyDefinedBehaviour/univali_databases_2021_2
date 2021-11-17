use anyhow::Result;
use async_trait::async_trait;
use sqlx::{
  mysql::{MySql, MySqlRow},
  Pool, Row,
};

use crate::domain::{cities, contract};

pub(super) struct CityRepository {
  pub pool: Pool<MySql>,
}

fn row_to_city(row: MySqlRow) -> Result<cities::City, sqlx::Error> {
  Ok(cities::City {
    id: row.try_get("city_id")?,
    name: row.try_get("city_name")?,
    state: cities::State {
      id: row.try_get("state_id")?,
      name: row.try_get("state_name")?,
    },
  })
}

#[async_trait]
impl contract::CityRepository for CityRepository {
  async fn get_all(&self) -> Result<Vec<cities::City>> {
    let cities = sqlx::query(
      "
      SELECT 
        tab_city.id as city_id,
        tab_city.name as city_name,
        tab_state.id as state_id,
        tab_state.name as state_name
      FROM tab_city
      INNER JOIN tab_state
      ON tab_state.id = tab_city.state_id
      ORDER BY tab_city.name ASC
    ",
    )
    .try_map(row_to_city)
    .fetch_all(&self.pool)
    .await?;

    Ok(cities)
  }
}
