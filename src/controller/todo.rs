use crate::model::todo;
use crate::view::html;
use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/todo")]
async fn index(state: web::Data<AppState>) -> impl Responder {
    let todos = todo::get_todo_list(&state.pool).await;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::todo::render_index_page(&todos))
}
