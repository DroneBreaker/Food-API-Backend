use crate::db::Database;
use crate::models::{BuyPizzaRequest, UpdatePizza};
use actix_web::{
    get, patch, post,
    web::{Json, Path, Data},
    App, HttpResponse, HttpServer, Responder,
};
use validator::Validate;

mod db;
mod models;

#[get("/pizzas")]
async fn all_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas available.!")
}

#[post("/getpizza")]
async fn get_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("Pizza is {pizza_name}"))
        }
        Err(_) => HttpResponse::Ok().body("Pizza name required"),
    }
}

#[patch("/pizzas/{uuid}")]
async fn update_pizza(url: Path<UpdatePizza>) -> impl Responder {
    let uuid = url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Updating pizza with id {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("Error connecting to database");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(all_pizzas)
            .service(update_pizza)
            .service(get_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
