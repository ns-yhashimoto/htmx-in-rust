CREATE TABLE IF NOT EXISTS orders (
  order_id serial PRIMARY KEY,
  order_status TEXT NOT NULL
);
INSERT INTO orders(order_status) VALUES ('引き当て済み');
INSERT INTO orders(order_status) VALUES ('引き当て済み');
INSERT INTO orders(order_status) VALUES ('未回答');
INSERT INTO orders(order_status) VALUES ('未回答');