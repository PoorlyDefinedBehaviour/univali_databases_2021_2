use crate::domain::{
  condominiums::{dto, Address, City, Condominium, State},
  contract,
};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{
  mysql::{MySql, MySqlRow},
  Pool, Row,
};

pub(super) struct CondominiumRepository {
  pub pool: Pool<MySql>,
}

fn row_to_condominium(row: MySqlRow) -> Result<Condominium, sqlx::Error> {
  Ok(Condominium {
    id: row.try_get("condominium_id")?,
    name: row.try_get("condominium_name")?,
    cnpj: row.try_get("condominium_cnpj")?,
    address: Address {
      id: row.try_get("address_id")?,
      street: row.try_get("address_street")?,
      number: row.try_get("address_number")?,
      city: City {
        id: row.try_get("city_id")?,
        name: row.try_get("city_name")?,
        state: State {
          id: row.try_get("state_id")?,
          name: row.try_get("state_name")?,
        },
      },
    },
  })
}

impl CondominiumRepository {
  async fn get_by_id(&self, id: i32) -> Result<Option<Condominium>> {
    let record = sqlx::query(
      "
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
      ON tab_state.id = tab_city.state_id
      WHERE tab_condominium.id = ?
      LIMIT 1
      ",
    )
    .bind(id)
    .fetch_optional(&self.pool)
    .await?;

    match record {
      None => Ok(None),
      Some(row) => Ok(Some(row_to_condominium(row)?)),
    }
  }
}

#[async_trait]
impl contract::CondominiumRepository for CondominiumRepository {
  async fn get_all(&self) -> Result<Vec<Condominium>> {
    let condominiums = sqlx::query(
      "
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
      ON tab_state.id = tab_city.state_id
      ORDER BY tab_condominium.created_at DESC
      ",
    )
    .try_map(row_to_condominium)
    .fetch_all(&self.pool)
    .await?;

    Ok(condominiums)
  }

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

  async fn update(
    &self,
    condominium_id: i32,
    data: dto::condominium::Create,
  ) -> Result<Condominium> {
    let mut tx = self.pool.begin().await?;

    sqlx::query!(
      r#"
      UPDATE tab_condominium
      INNER JOIN tab_address
      ON tab_address.id = tab_condominium.id
      SET 
        tab_address.street = ?,
        tab_address.number = ?,
        tab_address.city_id = ?
      WHERE tab_condominium.id = ?
      "#,
      data.address.street,
      data.address.number,
      data.address.city_id,
      condominium_id,
    )
    .execute(&mut tx)
    .await?;

    sqlx::query!(
      r#"
      UPDATE tab_condominium
      SET 
        name = ?,
        cnpj = ?
      WHERE id = ?
      "#,
      data.name,
      data.cnpj,
      condominium_id
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    let condominium = self.get_by_id(condominium_id).await?.unwrap();

    Ok(condominium)
  }

  async fn delete(&self, condominium_id: i32) -> Result<()> {
    let _ = sqlx::query!("DELETE FROM tab_condominium WHERE id = ?", condominium_id)
      .execute(&self.pool)
      .await?;

    Ok(())
  }
}
