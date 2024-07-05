use tera::Tera;
use actix_web::{get, web, HttpResponse, Responder};
use crate::view::html;
use crate::model::order;

#[get("/order")]
async fn index(tera: web::Data<Tera>) -> impl Responder {
    let orders = order::get_order_balance_list();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::order::render_index_page(&tera, &orders))
}
