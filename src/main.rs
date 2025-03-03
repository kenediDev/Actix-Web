use actix_web::{
    App, HttpRequest, HttpServer, Responder,
    web::{self, Data},
};
use std::sync::Mutex;

struct Counter {
    counter: i32,
}

async fn index(data: Data<Mutex<Counter>>) -> impl Responder {
    let mut d = data.lock().unwrap();
    d.counter += 1;
    let counter = d.counter;
    format!("{counter}")
}

async fn extra(req: HttpRequest) -> impl Responder {
    let data = req.app_data::<Data<Mutex<Counter>>>().unwrap();
    let mut c = data.lock().unwrap();
    c.counter += 1;
    let counter = c.counter;
    format!("{counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = Data::new(Mutex::new(Counter { counter: 0 }));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .route("/", web::get().to(index))
            .route("/extra", web::get().to(extra))
    })
    .bind(("localhost", 4000))?
    .run()
    .await
}
