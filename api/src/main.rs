use actix_web::web::{self, ServiceConfig};
use repository::postgres_order_repository::PostgresOrderRepository;
use repository::postgres_todo_repository::PostgresTodoRepository;
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let config = move |cfg: &mut ServiceConfig| {
        let todo_repository = web::Data::new(PostgresTodoRepository::new(pool.clone()));
        let order_repository = web::Data::new(PostgresOrderRepository::new(pool.clone()));

        cfg.service(controller::root::index);
        cfg.configure(controller::order::service::<PostgresOrderRepository>);
        cfg.configure(controller::todo::service::<PostgresTodoRepository>);
        cfg.app_data(todo_repository);
        cfg.app_data(order_repository);
    };

    Ok(config.into())
}
