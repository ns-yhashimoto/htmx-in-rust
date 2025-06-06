use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use model::order::{self, OrderRepository};
use serde::Deserialize;
use view::html;

pub fn service<R: OrderRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/order")
            .route("", web::get().to(index::<R>))
            .route("/search", web::get().to(search::<R>)),
    );
}

async fn index<R: OrderRepository>(repos: web::Data<R>) -> actix_web::Result<HttpResponse> {
    let orders = order::get_order_balance_list(repos.as_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html::order::render_index_page(&orders)))
}

#[derive(Deserialize)]
struct SearchQuery {
    status: String,
}

async fn search<R: OrderRepository>(
    query: web::Query<SearchQuery>,
    repos: web::Data<R>,
) -> actix_web::Result<HttpResponse> {
    let orders = if &query.status != "" {
        order::search_order_balance(repos.as_ref(), &query.status)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?
    } else {
        order::get_order_balance_list(repos.as_ref())
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html::order::render_order_rows(&orders)))
}
