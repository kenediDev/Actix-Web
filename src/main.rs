use actix_web::{App, HttpServer, Result, get, web};

#[get("/profile/{username}/{email}")]
async fn index(data: web::Path<(String, String)>) -> Result<String> {
    let (username, email) = data.into_inner();
    Ok(format!("Username : {} \nEmail : {}", username, email))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("localhost", 4000))?
        .run()
        .await
}
