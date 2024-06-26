use std::fmt::format;

// use crate::db::Database;
use crate::db:: {pizza_data_traits::PizzaDataTrait, Database};
use crate::models::{BuyPizzaRequest, Pizza, UpdatePizza};
use actix_web::{
    get, patch, post,
    web::{Json, Path, Data},
    App, HttpResponse, HttpServer, Responder,
};
use error::PizzaError;
use surrealdb::sql::Uuid;
use validator::Validate;

mod db;
mod models;
mod error;

#[get("/pizzas")]
async fn all_pizzas(db: Data<Database>) -> Result<Json<Vec<Pizza>>, PizzaError> {
    let pizzas = Database::get_all_pizzas(&db).await;

    match pizzas {
        Some(found_pizzas) => Ok(Json(found_pizzas)),
        None => Err(PizzaError::NoPizzasFound),
    }
}

#[post("/getpizza")]
async fn get_pizza(body: Json<BuyPizzaRequest>, db: Data<Database>) -> Result<Json<Pizza>, PizzaError> {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_pizza = Database::create_pizza(&db, Pizza::new(pizza_name, String::from(new_uuid))).await;

            // HttpResponse::Ok().body(format!("Pizza is {pizza_name}"))
            match new_pizza {
                Some(created) => {
                    Ok(Json(created))
                },
                None => Err(PizzaError::PizzaCreatedFailure),
            }
        }
        Err(_) => Err(PizzaError::PizzaCreatedFailure),
    }
}

#[patch("/pizzas/{uuid}")]
async fn update_pizza(url: Path<UpdatePizza>, db: Data<Database>) -> Result<Json<Pizza>, PizzaError> {
    let uuid = url.into_inner().uuid;
    let updated_result = Database::update_pizza(&db, uuid).await;
    
    match updated_result {
        Some(updated) => Ok(Json(updated)),
        None => Err(PizzaError::NoSuchPizzaFound),
    }
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
