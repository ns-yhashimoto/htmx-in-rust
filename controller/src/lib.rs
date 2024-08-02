use sqlx::PgPool;

pub mod order;
pub mod root;
pub mod todo;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}
