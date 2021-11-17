-- We could fetch all brazilian cities from somewhere,
-- but these will be enough for testing purposes.

INSERT INTO tab_state (id, name)
VALUES
  (1, "Santa Catarina"),
  (2, "Paraná"),
  (3, "São Paulo");

INSERT INTO tab_city (name, state_id)
VALUES 
  ("Itajaí", 1),
  ("Tijucas", 1),
  ("Curitiba", 2),
  ("Londrina", 2),
  ("São Paulo", 3),
  ("Guarulhos", 3);
