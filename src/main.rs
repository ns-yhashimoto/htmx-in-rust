use htmx_in_rust;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    htmx_in_rust::run_server().await
}