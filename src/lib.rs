use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};

pub fn run(tcp_listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(tcp_listener)?
        .run();
    Ok(server)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
