use tera::Tera;
use actix_web::{get, web, HttpResponse, Responder};
use crate::{model::order::get_order_balance_list, view::html};

#[get("/order")]
async fn order(tera: web::Data<Tera>) -> impl Responder {
    let orders = get_order_balance_list();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::render_order_page(&tera, &orders))
}
