use crate::model::todo;
use crate::view::html;
use crate::AppState;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[get("/todo")]
async fn index(state: web::Data<AppState>) -> impl Responder {
    let todos = todo::get_list(&state.pool).await;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::todo::render_index_page(&todos))
}

#[derive(Deserialize)]
struct TodoCreate {
    content: String,
}

#[post("/todo")]
async fn index_post(form: web::Form<TodoCreate>, state: web::Data<AppState>) -> impl Responder {
    let todo = todo::create(&state.pool, &form.content).await;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::todo::render_items(&vec![todo]))
}

#[post("/todo/{id}/done")]
async fn index_post_done(path: web::Path<(i32,)>, state: web::Data<AppState>) -> impl Responder {
    let result = todo::update_as_done(&state.pool, &path.into_inner().0).await;

    match result {
        Ok(todo) => HttpResponse::Ok()
            .content_type("text/html")
            .body(html::todo::render_items(&vec![todo])),
        Err(e) => {
            println!("{}", e);
            HttpResponse::NotFound().finish()
        }
    }
}

#[delete("/todo/{id}")]
async fn index_delete(path: web::Path<(i32,)>, state: web::Data<AppState>) -> impl Responder {
    let result = todo::delete(&state.pool, &path.into_inner().0).await;

    match result {
        Ok(_) => HttpResponse::SeeOther()
            .append_header(("Location", "/todo"))
            .finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
