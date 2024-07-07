use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    htmx_in_rust::run_server(pool)
}
