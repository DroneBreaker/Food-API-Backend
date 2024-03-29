use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message = "Pizza name required"))]
    pub pizza_name: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdatePizza {
    pub uuid: String,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct Pizza {
    pub pizza_name: String,
    pub uuid: String
}

impl Pizza {
    pub fn new(pizza_name: String, uuid: String) -> Pizza {
        Pizza {
            pizza_name,
            uuid
        }
    }
}
