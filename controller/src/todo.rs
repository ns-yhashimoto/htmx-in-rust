use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use model::todo;
use model::todo::TodoRepository;
use serde::Deserialize;
use view::html;

pub fn service<R: TodoRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/todo")
            .route("", web::get().to(index::<R>))
            .route("", web::post().to(index_post::<R>))
            .route("/{id}/done", web::post().to(index_post_done::<R>))
            .route("/{id}", web::delete().to(index_delete::<R>)),
    );
}

async fn index<R: TodoRepository>(repos: web::Data<R>) -> impl Responder {
    let todos = todo::get_list(repos.get_ref()).await;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::todo::render_index_page(&todos))
}

#[derive(Deserialize)]
struct TodoCreate {
    content: String,
}

async fn index_post<R: TodoRepository>(
    form: web::Form<TodoCreate>,
    repos: web::Data<R>,
) -> impl Responder {
    let todo = todo::create(repos.as_ref(), &form.content).await;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::todo::render_items(&vec![todo]))
}

async fn index_post_done<R: TodoRepository>(
    path: web::Path<(i32,)>,
    repos: web::Data<R>,
) -> impl Responder {
    let result = todo::update_as_done(repos.as_ref(), &path.into_inner().0).await;

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

async fn index_delete<R: TodoRepository>(
    path: web::Path<(i32,)>,
    repos: web::Data<R>,
) -> impl Responder {
    let result = todo::delete(repos.as_ref(), &path.into_inner().0).await;

    match result {
        Ok(_) => HttpResponse::SeeOther()
            .append_header(("Location", "/todo"))
            .finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
