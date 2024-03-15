use actix_web::{get, patch, post, web::get, App, HttpResponse, HttpServer, Responder};

#[get("/pizzas")]
async fn all_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas available.!")
}

#[post("/buypizza")]
async fn get_pizza() -> impl Responder {
    HttpResponse::Ok().body("Buying pizza")
}

#[patch("/pizzas/{uuid}")]
async fn update_pizza() -> impl Responder {
    HttpResponse::Ok().body("Updating pizza")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(all_pizzas)
            .service(update_pizza)
            .service(get_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
