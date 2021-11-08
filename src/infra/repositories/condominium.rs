use crate::domain::condominiums::{contract, dto, Address, City, Condominium, State};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{mysql::MySql, Pool};

pub struct CondominiumRepository {
  pool: Pool<MySql>,
}

impl CondominiumRepository {
  async fn get_by_id(&self, id: i32) -> Result<Option<Condominium>> {
    let record = sqlx::query!(
      r#"
      SELECT
        tab_condominium.id as condominium_id,
        tab_condominium.name as condominium_name,
        tab_condominium.cnpj as condominium_cnpj,
        tab_address.id as address_id,
        tab_address.street as address_street,
        tab_address.number as address_number,
        tab_city.id as city_id,
        tab_city.name as city_name,
        tab_state.id as state_id,
        tab_state.name as state_name
      FROM tab_condominium
      INNER JOIN tab_address
      ON tab_address.id = tab_condominium.address_id
      INNER JOIN tab_city
      ON tab_city.id = tab_address.city_id
      INNER JOIN tab_state
      ON tab_state.id = tab_city.id
      WHERE tab_condominium.id = ?
      LIMIT 1
      "#,
      id,
    )
    .fetch_optional(&self.pool)
    .await?;

    match record {
      None => Ok(None),
      Some(row) => Ok(Some(Condominium {
        id: row.condominium_id,
        name: row.condominium_name,
        cnpj: row.condominium_cnpj,
        address: Address {
          id: row.address_id,
          street: row.address_street,
          number: row.address_number,
          city: City {
            id: row.city_id,
            name: row.city_name,
            state: State {
              id: row.state_id,
              name: row.state_name,
            },
          },
        },
      })),
    }
  }
}

#[async_trait]
impl contract::repositories::CondominiumRepository for CondominiumRepository {
  async fn create(&self, data: dto::condominium::Create) -> Result<Condominium> {
    let mut tx = self.pool.begin().await?;

    let insert_address_result = sqlx::query!(
      r#"
      INSERT INTO tab_address (street, number, city_id)
      VALUES (?, ?, ?)
      "#,
      data.address.street,
      data.address.number,
      data.address.city_id,
    )
    .execute(&mut tx)
    .await?;

    let insert_condominium_result = sqlx::query!(
      r#"
      INSERT INTO tab_condominium (name, cnpj, address_id)
      VALUES (?, ?, ?)
      "#,
      data.name,
      data.cnpj,
      insert_address_result.last_insert_id(),
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    let condominium = self
      .get_by_id(insert_condominium_result.last_insert_id() as i32)
      .await?
      .unwrap();

    Ok(condominium)
  }
}
