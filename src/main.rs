use actix_web::{App, HttpServer, Responder, web};

async fn home() -> impl Responder {
    "Kenedy Dev On Youtube"
}

async fn about() -> impl Responder {
    "ini adalah url About"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/app")
                .route("/home", web::get().to(home))
                .route("/about", web::get().to(about)),
        )
    })
    .bind(("localhost", 4000))?
    .run()
    .await
}
