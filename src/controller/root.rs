use crate::view::html;
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::root::render_index_page())
}
