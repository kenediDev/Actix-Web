use actix_web::{App, HttpServer, Responder, web};
use std::sync::Mutex;

struct Person {
    fullName: String,
    age: Mutex<i32>,
    is_male: bool,
}

async fn index(data: web::Data<Person>) -> impl Responder {
    let fullName = &data.fullName;
    let mut age = data.age.lock().unwrap();
    let is_male = &data.is_male;
    *age += 1;
    format!("Full Name : {fullName} \n Age {age} \n Is Male {is_male}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let person = web::Data::new(Person {
        fullName: String::from("Kenedy Dev On Youtube"),
        age: Mutex::new(0),
        is_male: true,
    });
    HttpServer::new(move || {
        App::new()
            .app_data(person.clone())
            .route("/", web::get().to(index))
    })
    .bind(("localhost", 4000))?
    .run()
    .await
}
