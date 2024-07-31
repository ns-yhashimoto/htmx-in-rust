use crate::model::todo;
use crate::model::todo::TodoRepository;
use crate::view::html;
use actix_web::{
    delete, get, post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use serde::Deserialize;

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(index)
        .service(index_post)
        .service(index_post_done)
        .service(index_delete);
}

type Repository = web::Data<Box<dyn TodoRepository>>;

#[get("/todo")]
async fn index(repos: Repository) -> impl Responder {
    let todos = todo::get_list(&repos).await;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::todo::render_index_page(&todos))
}

#[derive(Deserialize)]
struct TodoCreate {
    content: String,
}

#[post("/todo")]
async fn index_post(form: web::Form<TodoCreate>, repos: Repository) -> impl Responder {
    let todo = todo::create(&repos, &form.content).await;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::todo::render_items(&vec![todo]))
}

#[post("/todo/{id}/done")]
async fn index_post_done(path: web::Path<(i32,)>, repos: Repository) -> impl Responder {
    let result = todo::update_as_done(&repos, &path.into_inner().0).await;

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
async fn index_delete(path: web::Path<(i32,)>, repos: Repository) -> impl Responder {
    let result = todo::delete(&repos, &path.into_inner().0).await;

    match result {
        Ok(_) => HttpResponse::SeeOther()
            .append_header(("Location", "/todo"))
            .finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
