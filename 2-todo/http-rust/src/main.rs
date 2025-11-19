use actix_web::{App, HttpResponse, HttpServer, Responder, web};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hi ser")
}

async fn hey() -> impl Responder {
    HttpResponse::Ok().body("hi ser")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/hey", web::get().to(hey))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
