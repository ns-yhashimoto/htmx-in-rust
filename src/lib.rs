use actix_web::web::{self, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

mod controller {
    pub mod root;
    pub mod order;
}

mod model {
    pub mod order;
}

mod view {
    pub mod html;
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

pub async fn run_server(pool: PgPool) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let config = move |cfg: &mut ServiceConfig| {
        let state = web::Data::new(AppState { pool });
        
        cfg.service(controller::root::index);
        cfg.service(controller::order::index);
        cfg.service(controller::order::search);
        cfg.app_data(state);
    };

    Ok(config.into())
}