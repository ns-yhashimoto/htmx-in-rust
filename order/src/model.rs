use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub type OrderError = String;
pub type OrderResult<T> = Result<T, OrderError>;

#[async_trait::async_trait]
pub trait OrderRepository: Send + Sync + 'static {
    async fn search_order_balance(&self, status: &String) -> OrderResult<Vec<OrderBalance>>;
    async fn get_order_balance_list(&self) -> OrderResult<Vec<OrderBalance>>;
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct OrderBalance {
    pub order_id: i32,
    pub order_status: String,
}

pub async fn search_order_balance(
    repos: &impl OrderRepository,
    status: &String,
) -> OrderResult<Vec<OrderBalance>> {
    repos.search_order_balance(status).await
}

pub async fn get_order_balance_list(
    repos: &impl OrderRepository,
) -> OrderResult<Vec<OrderBalance>> {
    repos.get_order_balance_list().await
}
