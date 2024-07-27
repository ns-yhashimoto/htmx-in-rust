CREATE TABLE IF NOT EXISTS orders (
  order_id serial PRIMARY KEY,
  order_status TEXT NOT NULL
);
INSERT INTO orders(order_status) VALUES ('引き当て済み');
INSERT INTO orders(order_status) VALUES ('引き当て済み');
INSERT INTO orders(order_status) VALUES ('未回答');
INSERT INTO orders(order_status) VALUES ('未回答');

CREATE TABLE IF NOT EXISTS todos (
  id serial PRIMARY KEY,
  content TEXT NOT NULL,
  completed_on TIMESTAMP
);
