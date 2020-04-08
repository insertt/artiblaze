use std::io;

use actix_web::{HttpRequest, HttpServer, Responder, web, App};

async fn index(req: HttpRequest) -> impl Responder {
    format!("Hello from {}!", req.connection_info().remote().unwrap())
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
