use actix_web::{post, web, App, HttpResponse, HttpServer};
use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
struct Customer {
    #[validate(length(min = 5, max = 100))]
    name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 10, max = 12))]
    phone: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct Item {
    #[validate(length(min = 2))]
    product_id: String,
    #[validate(range(min = 1))]
    quantity: u32,
    #[validate(range(min = 0.0))]
    price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
enum Shipping {
    Standard,
    Express,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct Order {
    #[validate(nested)]
    customer: Customer,
    #[validate(nested, length(min = 1, message = "Order must have at least one item"))]
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
            HttpResponse::BadRequest().json(e.errors())
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
