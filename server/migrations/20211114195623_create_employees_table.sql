CREATE TABLE IF NOT EXISTS tab_shift (
  id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO tab_shift (name)
VALUES
  ('Manhã'),
  ('Tarde'),
  ('Noite'),
  ('Integral');

CREATE TABLE IF NOT EXISTS tab_role (
  id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO tab_role (name)
VALUES 
  ('Zelador'),
  ('Porteiro');

CREATE TABLE IF NOT EXISTS tab_employee (
  id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL,
  cpf VARCHAR(11) NOT NULL,
  wage_in_cents INT NOT NULL,
  works_at_condominium_id INT NOT NULL,
  shift_id INT NOT NULL,
  role_id INT NOT NULL,
  address_id INT NOT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(cpf, works_at_condominium_id),
  FOREIGN KEY(works_at_condominium_id) REFERENCES tab_condominium(id) 
    ON DELETE CASCADE,
  FOREIGN KEY(shift_id) REFERENCES tab_shift(id),
  FOREIGN KEY(role_id) REFERENCES tab_role(id),
  FOREIGN KEY(address_id) REFERENCES tab_address(id)
);