use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ValidationError {
  pub field: &'static str,
  pub message: String,
}
