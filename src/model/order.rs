use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres};

#[derive(Serialize, Deserialize, FromRow)]
pub struct OrderBalance {
    order_id: i32,
    order_status: String,
}

pub async fn search_order_balance(pool: &Pool<Postgres>, status: &String) -> Vec<OrderBalance> {
    sqlx::query_as("SELECT order_id, order_status FROM orders WHERE order_status = $1")
        .bind(status)
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn get_order_balance_list(pool: &Pool<Postgres>) -> Vec<OrderBalance> {
    sqlx::query_as("SELECT order_id, order_status FROM orders")
        .fetch_all(pool)
        .await
        .unwrap()
}
