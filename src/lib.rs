use actix_web::web::{self, ServiceConfig};
use repository::postgres_todo_repository::PostgresTodoRepository;
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

mod controller {
    pub mod order;
    pub mod root;
    pub mod todo;
}

mod model {
    pub mod order;
    pub mod todo;
}

mod view {
    pub mod html;
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

mod repository;

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
        cfg.service(controller::order::index);
        cfg.service(controller::order::search);
        cfg.configure(controller::todo::service::<PostgresTodoRepository>);
        cfg.app_data(todo_repository);
        cfg.app_data(state);
    };

    Ok(config.into())
}
