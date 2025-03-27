// Using serde_valid for validation (Not as good/flexible as validator)
// https://docs.rs/serde_valid/latest/serde_valid/

use actix_web::{post, web, App, HttpResponse, HttpServer};
use serde_valid::Validate;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Validate)]
struct Customer {
    #[validate(min_length = 5)]
    name: String,
    #[validate(min_length = 11)]
    email: String,
    #[validate(min_length = 10)]
    phone: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct Item {
    #[validate(min_length = 2)]
    product_id: String,
    #[validate(minimum = 1)]
    quantity: u32,
    #[validate(minimum = 0.0)]
    price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
enum Shipping {
    Standard,
    Express,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct Order {
    #[validate]
    customer: Customer,
    #[validate(min_items = 1)]
    items: Vec<Item>,
    shipping: Shipping,
    discount_code: String,
}
#[post("/order")]
async fn new_order(order: web::Json<Order>) -> HttpResponse {
    match order.0.validate() {
        Ok(_) => {
            println!("Order: {:?}", order);
            HttpResponse::Ok().json(order)
        } 
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::BadRequest().json(e)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(new_order)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/*
With serde_valid, you can use these validation types:
For strings:
min_length
max_length
pattern (regex pattern)
For numbers:
minimum
maximum
exclusive_minimum
exclusive_maximum
multiple_of
For collections (Vec, etc.):
min_items
max_items
unique_items
For objects:
min_properties
max_properties

*/