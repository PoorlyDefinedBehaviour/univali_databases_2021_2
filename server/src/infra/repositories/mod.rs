use sqlx::{mysql::MySql, Pool};

use crate::domain::contract::Database;

mod city;
mod condominium;
mod employee;

pub fn new(pool: Pool<MySql>) -> Database {
  Database {
    condominium: Box::new(condominium::CondominiumRepository { pool: pool.clone() }),
    employee: Box::new(employee::EmployeeRepository { pool: pool.clone() }),
    city: Box::new(city::CityRepository { pool: pool.clone() }),
  }
}
