use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use crate::model::{self as model, OrderRepository};
use serde::Deserialize;
use crate::view::html;

pub fn service<R: OrderRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/order")
            .route("", web::get().to(index::<R>))
            .route("/search", web::get().to(search::<R>)),
    );
}

async fn index<R: OrderRepository>(repos: web::Data<R>) -> impl Responder {
    let orders = model::get_order_balance_list(repos.as_ref()).await.unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::render_index_page(&orders))
}

#[derive(Deserialize)]
struct SearchQuery {
    status: String,
}

async fn search<R: OrderRepository>(
    query: web::Query<SearchQuery>,
    repos: web::Data<R>,
) -> impl Responder {
    let orders = if &query.status != "" {
        model::search_order_balance(repos.as_ref(), &query.status)
            .await
            .unwrap()
    } else {
        model::get_order_balance_list(repos.as_ref()).await.unwrap()
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::render_order_rows(&orders))
}
