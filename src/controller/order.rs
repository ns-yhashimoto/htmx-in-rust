use crate::model::order;
use crate::view::html;
use crate::AppState;
use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use serde::Deserialize;

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/order")
            .route("", web::get().to(index))
            .route("/search", web::get().to(search)),
    );
}

async fn index(state: web::Data<AppState>) -> impl Responder {
    let orders = order::get_order_balance_list(&state.pool).await;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::order::render_index_page(&orders))
}

#[derive(Deserialize)]
struct SearchQuery {
    status: String,
}

async fn search(query: web::Query<SearchQuery>, state: web::Data<AppState>) -> impl Responder {
    let orders = if &query.status != "" {
        order::search_order_balance(&state.pool, &query.status).await
    } else {
        order::get_order_balance_list(&state.pool).await
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::order::render_order_rows(&orders))
}
