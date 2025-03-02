use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

#[post("/created")]
async fn created() -> impl Responder {
    HttpResponse::Ok().body("Created")
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Kenedy Dev On Youtube")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(created).service(home))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
