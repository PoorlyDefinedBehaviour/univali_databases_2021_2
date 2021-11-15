use sqlx::{mysql::MySql, Pool};

use crate::domain::contract::Database;

mod condominium;
mod employee;

pub fn new(pool: Pool<MySql>) -> Database {
  Database {
    condominium: Box::new(condominium::CondominiumRepository { pool: pool.clone() }),
    employee: Box::new(employee::EmployeeRepository { pool: pool.clone() }),
  }
}
