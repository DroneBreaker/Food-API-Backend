use crate::models::{Pizza};
use crate::{db::Database};
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::{Error};

#[async_trait]
pub trait PizzaDataTrait {
    async fn get_all_pizzas(db: &Data<Database>) -> Option<Vec<Pizza>>;
    async fn create_pizza(db: &Data<Database>, new_pizza: Pizza) -> Option<Pizza>;
    async fn update_pizza(db: &Data<Database>, uuid: String) -> Option<Pizza>;
}

#[async_trait]
impl PizzaDataTrait for Database {
     // Getting all pizzas
    async fn get_all_pizzas(db: &Data<Database>) -> Option<Vec<Pizza>> {
        let result = db.client.select("pizza").await;

        match result {
            Ok(all_pizzas) => Some(all_pizzas),
            Err(_) => None,
        }
    }

     // Create pizzas
    async fn create_pizza(db: &Data<Database>, new_pizza: Pizza) -> Option<Pizza> {
        let created_pizza = db
            .client
            .create(("pizza", new_pizza.uuid.clone()))
            .content(new_pizza)
            .await;

        match created_pizza {
            Ok(created) => created,
            Err(_) => None
        }
    }

    // Getting a pizza by UUID
    async fn update_pizza(db: &Data<Database>, uuid: String) -> Option<Pizza> {
        let find_pizza: Result<Option<Pizza>, Error> = db.client.select(("pizza", &uuid)).await;

        match find_pizza {
            Ok(found) => {
                match found {
                    Some(_found_pizza) => {
                        let updated_pizza = db.client.update(("pizza", &uuid)).merge(Pizza {
                            pizza_name:String::from("sold"),
                            uuid
                        }).await;

                        match updated_pizza {
                            Ok(updated) => updated,
                            Err(_) => None
                        }
                    },

                    None => None,
                }
            },

            Err(_) => None,
        }
    }
}