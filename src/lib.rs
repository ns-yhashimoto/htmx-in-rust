use std::net::Ipv4Addr;
use actix_web::{App, HttpServer};

mod controller {
    pub mod root;
    pub mod order;
}

mod model {
    pub mod order;
}

mod view {
    pub mod html;
}

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(controller::root::index)
            .service(controller::order::index)
            .service(controller::order::search)
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}