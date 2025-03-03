use actix_web::{App, HttpServer, Responder, get, web};

struct Person {
    fullName: String,
    age: u8,
    subscribe: bool,
}

#[get("/")]
async fn index(data: web::Data<Person>) -> impl Responder {
    let fullName = &data.fullName;
    let age = &data.age;
    let subscribe = &data.subscribe;
    format!("Name : {fullName}, Age : {age}, Don't forgot subscribe : {subscribe}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Person {
                fullName: String::from("Kenedy Dev On Youtube"),
                age: 18,
                subscribe: true,
            }))
            .service(index)
    })
    .bind(("localhost", 4000))?
    .run()
    .await
}
