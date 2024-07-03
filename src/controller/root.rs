
use tera::Tera;
use actix_web::{get, web, HttpResponse, Responder};
use crate::view::html;

#[get("/")]
async fn hello(tera: web::Data<Tera>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::render_index_page(&tera))
}
