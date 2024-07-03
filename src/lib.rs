use std::net::Ipv4Addr;
use actix_web::{web, App, HttpServer};
use tera::Tera;

mod controller {
    pub mod root;
}

mod view {
    pub mod html;
}

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("src/view/templates/**/*.html").unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .service(controller::root::hello)
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}