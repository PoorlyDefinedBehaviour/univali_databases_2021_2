use cpf_cnpj::cnpj;
use std::convert::TryFrom;

use super::ValidationError;

#[derive(Debug, PartialEq)]
pub struct Cnpj(String);

impl Cnpj {
  pub fn into_inner(self) -> String {
    self.0
  }
}

impl TryFrom<String> for Cnpj {
  type Error = ValidationError;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    if cnpj::validate(&value) {
      // This is O(1) since a Cnpj cannot be longer than 14 characters.
      let value: String = value.chars().filter(|c| c.is_digit(10)).collect();

      Ok(Self(value))
    } else {
      Err(ValidationError {
        field: "cnpj",
        message: format!("cnpj {} is not valid", value),
      })
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_characters_that_arent_digits() {
    assert_eq!(
      Ok(Cnpj(String::from("53938054000169"))),
      Cnpj::try_from(String::from("53.938.054/0001-69"))
    );
  }

  #[test]
  fn rejects_invalid_cnpjs() {
    let tests = vec!["x", "99999999999", "255.248.930-333"];

    for cnpj in tests {
      assert_eq!(
        Err(ValidationError {
          field: "cnpj",
          message: format!("cnpj {} is not valid", cnpj),
        }),
        Cnpj::try_from(String::from(cnpj))
      );
    }
  }

  #[test]
  fn accepts_valid_cnpjs() {
    let tests = vec!["53.938.054/0001-69", "36.002.518/0001-01", "36002518000101"];

    for cnpj in tests {
      let actual = Cnpj::try_from(String::from(cnpj));

      assert!(matches!(actual, Ok(_)));
    }
  }
}
