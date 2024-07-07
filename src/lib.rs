use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

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

pub fn run_server() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(controller::root::index);
        cfg.service(controller::order::index);
        cfg.service(controller::order::search);
    };

    Ok(config.into())
}