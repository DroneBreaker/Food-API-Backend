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
