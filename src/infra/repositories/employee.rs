use std::convert::TryFrom;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{
  mysql::{MySql, MySqlRow},
  Pool, Row,
};

use crate::domain::{contract, employees, value_objects::cpf::Cpf};

pub(super) struct EmployeeRepository {
  pub pool: Pool<MySql>,
}

fn row_to_employee(row: MySqlRow) -> Result<employees::Employee, sqlx::Error> {
  let cpf: String = row.try_get("employee_cpf")?;

  Ok(employees::Employee {
    id: row.try_get("employee_id")?,
    name: row.try_get("employee_name")?,
    cpf: Cpf::try_from(cpf).unwrap(),
    wage_in_cents: row.try_get("employee_wage_in_cents")?,
    shift: employees::Shift {
      id: row.try_get("shift_id")?,
      name: row.try_get("shift_name")?,
    },
    role: employees::Role {
      id: row.try_get("role_id")?,
      name: row.try_get("role_name")?,
    },
    address: employees::Address {
      id: row.try_get("address_id")?,
      street: row.try_get("address_street")?,
      number: row.try_get("address_number")?,
      city: employees::City {
        id: row.try_get("city_id")?,
        name: row.try_get("city_name")?,
        state: employees::State {
          id: row.try_get("state_id")?,
          name: row.try_get("state_name")?,
        },
      },
    },
  })
}

impl EmployeeRepository {
  async fn get_by_id(&self, id: i32) -> Result<Option<employees::Employee>> {
    let row = sqlx::query(
      "
      SELECT 
        tab_employee.id as employee_id,
        tab_employee.name as employee_name,
        tab_employee.cpf employee_cpf,
        tab_employee.wage_in_cents employee_wage_in_cents,
        tab_shift.id as shift_id,
        tab_shift.name as shift_name,
        tab_role.id as role_id,
        tab_role.name as role_name,
        tab_address.id as address_id,
        tab_address.street as address_street,
        tab_address.number as address_number,
        tab_city.id as city_id,
        tab_city.name as city_name,
        tab_state.id as state_id,
        tab_state.name as state_name
      FROM tab_employee
      INNER JOIN tab_shift
      ON tab_shift.id = tab_employee.shift_id
      INNER JOIN tab_role
      ON tab_role.id = tab_employee.role_id
      INNER JOIN tab_address
      ON tab_address.id = tab_employee.address_id
      INNER JOIN tab_city 
      ON tab_city.id = tab_address.city_id
      INNER JOIN tab_state
      ON tab_state.id = tab_city.state_id
      WHERE tab_employee.id = ?
      ",
    )
    .bind(id)
    .fetch_optional(&self.pool)
    .await?;

    match row {
      None => Ok(None),
      Some(row) => Ok(Some(row_to_employee(row)?)),
    }
  }
}

#[async_trait]
impl contract::EmployeeRepository for EmployeeRepository {
  async fn get_all(&self) -> Result<Vec<employees::Employee>> {
    let employees = sqlx::query(
      "
      SELECT 
        tab_employee.id as employee_id,
        tab_employee.name as employee_name,
        tab_employee.cpf employee_cpf,
        tab_employee.wage_in_cents employee_wage_in_cents,
        tab_shift.id as shift_id,
        tab_shift.name as shift_name,
        tab_role.id as role_id,
        tab_role.name as role_name,
        tab_address.id as address_id,
        tab_address.street as address_street,
        tab_address.number as address_number,
        tab_city.id as city_id,
        tab_city.name as city_name,
        tab_state.id as state_id,
        tab_state.name as state_name
      FROM tab_employee
      INNER JOIN tab_shift
      ON tab_shift.id = tab_employee.shift_id
      INNER JOIN tab_role
      ON tab_role.id = tab_employee.role_id
      INNER JOIN tab_address
      ON tab_address.id = tab_employee.address_id
      INNER JOIN tab_city 
      ON tab_city.id = tab_address.city_id
      INNER JOIN tab_state
      ON tab_state.id = tab_city.state_id
      ORDER BY tab_employee.created_at DESC
    ",
    )
    .try_map(row_to_employee)
    .fetch_all(&self.pool)
    .await?;

    Ok(employees)
  }

  async fn create(&self, data: employees::dto::Create) -> Result<employees::Employee> {
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

    let insert_employee_result = sqlx::query!(
      "
      INSERT INTO tab_employee (
        name,
        cpf,
        wage_in_cents,
        works_at_condominium_id,
        shift_id,
        role_id, 
        address_id
      )
      VALUES (?, ?, ?, ?, ?, ?, ?)
      ",
      data.name,
      data.cpf.into_inner(),
      data.wage_in_cents,
      data.works_at_condominium_id,
      data.shift_id,
      data.role_id,
      insert_address_result.last_insert_id(),
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    let employee = self
      .get_by_id(insert_employee_result.last_insert_id() as i32)
      .await?;

    Ok(employee.unwrap())
  }

  async fn update(
    &self,
    employee_id: i32,
    data: employees::dto::Update,
  ) -> Result<employees::Employee> {
    let mut tx = self.pool.begin().await?;

    sqlx::query!(
      r#"
      UPDATE tab_employee
      INNER JOIN tab_address
      ON tab_address.id = tab_employee.address_id
      SET 
        tab_address.street = ?,
        tab_address.number = ?,
        tab_address.city_id = ?
      WHERE tab_employee.id = ?
      "#,
      data.address.street,
      data.address.number,
      data.address.city_id,
      employee_id,
    )
    .execute(&mut tx)
    .await?;

    sqlx::query!(
      "
      UPDATE tab_employee 
      SET
        name = ?,
        cpf = ?,
        wage_in_cents = ?,
        works_at_condominium_id = ?,
        shift_id = ?,
        role_id = ?
      WHERE tab_employee.id = ?
      ",
      data.name,
      data.cpf.into_inner(),
      data.wage_in_cents,
      data.works_at_condominium_id,
      data.shift_id,
      data.role_id,
      employee_id,
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    let employee = self.get_by_id(employee_id).await?;

    Ok(employee.unwrap())
  }

  async fn delete(&self, employee_id: i32) -> Result<()> {
    let mut tx = self.pool.begin().await?;

    let _ = sqlx::query!(
      "
      DELETE tab_employee, tab_address 
      FROM tab_employee 
      INNER JOIN tab_address
      ON tab_address.id = tab_employee.address_id
      WHERE tab_employee.id = ?
    ",
      employee_id
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(())
  }
}
