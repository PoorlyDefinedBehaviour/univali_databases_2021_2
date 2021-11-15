pub mod cpf;

#[derive(Debug, PartialEq)]
pub struct ValidationError {
  pub field: &'static str,
  pub message: String,
}
