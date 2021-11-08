pub struct Create {
  pub name: String,
  pub cnpj: String,
  pub address: Address,
}

pub struct Address {
  pub street: String,
  pub number: String,
  pub city_id: i32,
}
