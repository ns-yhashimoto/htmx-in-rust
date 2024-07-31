use actix_web::web::{self, ServiceConfig};
use repository::postgres_todo_repository::PostgresTodoRepository;
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

mod controller;
mod model;
mod repository;
mod view;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

pub async fn run_server(
    pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let config = move |cfg: &mut ServiceConfig| {
        let todo_repository = web::Data::new(PostgresTodoRepository::new(pool.clone()));
        let state = web::Data::new(AppState { pool });

        cfg.service(controller::root::index);
        cfg.configure(controller::order::service);
        cfg.configure(controller::todo::service::<PostgresTodoRepository>);
        cfg.app_data(todo_repository);
        cfg.app_data(state);
    };

    Ok(config.into())
}
