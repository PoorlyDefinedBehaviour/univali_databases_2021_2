use cpf_cnpj::cpf;
use std::convert::TryFrom;

use super::ValidationError;

#[derive(Debug, PartialEq)]
pub struct Cpf(String);

impl Cpf {
  pub fn into_inner(self) -> String {
    self.0
  }
}

impl TryFrom<String> for Cpf {
  type Error = ValidationError;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    if cpf::validate(&value) {
      Ok(Self(value))
    } else {
      Err(ValidationError {
        field: "cpf",
        message: format!("cpf {} is not valid", value),
      })
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn only_valid_cpfs_can_be_created() {
    let tests = vec![
      (
        String::from("31133244327"),
        Ok(Cpf(String::from("31133244327"))),
      ),
      (
        String::from("57568923541"),
        Ok(Cpf(String::from("57568923541"))),
      ),
      (
        String::from("17734532493"),
        Err(ValidationError {
          field: "cpf",
          message: String::from("cpf 17734532493 is not valid"),
        }),
      ),
      (
        String::from("00107918984"),
        Err(ValidationError {
          field: "cpf",
          message: String::from("cpf 00107918984 is not valid"),
        }),
      ),
      (
        String::from("aaaaaa"),
        Err(ValidationError {
          field: "cpf",
          message: String::from("cpf aaaaaa is not valid"),
        }),
      ),
      (
        String::from(""),
        Err(ValidationError {
          field: "cpf",
          message: String::from("cpf  is not valid"),
        }),
      ),
    ];

    for (cpf, expected) in tests {
      assert_eq!(expected, Cpf::try_from(cpf));
    }
  }
}
