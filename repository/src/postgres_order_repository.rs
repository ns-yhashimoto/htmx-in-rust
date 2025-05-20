use model::order::{OrderBalance, OrderRepository, OrderResult};

pub struct PostgresOrderRepository {
    pool: sqlx::PgPool,
}

impl PostgresOrderRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl OrderRepository for PostgresOrderRepository {
    async fn search_order_balance(&self, status: &String) -> OrderResult<Vec<OrderBalance>> {
        sqlx::query_as::<_, OrderBalance>(
            "SELECT order_id, order_status FROM orders WHERE order_status = $1",
        )
        .bind(status)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_order_balance_list(&self) -> OrderResult<Vec<OrderBalance>> {
        sqlx::query_as::<_, OrderBalance>("SELECT order_id, order_status FROM orders")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}