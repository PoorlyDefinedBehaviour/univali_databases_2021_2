use crate::domain::condominiums::contract::repositories::Database;
use sqlx::{mysql::MySql, Pool};

mod condominium;

pub fn new(pool: Pool<MySql>) -> Database {
  Database {
    condominium: Box::new(condominium::CondominiumRepository { pool: pool.clone() }),
  }
}
