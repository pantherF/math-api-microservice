use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Numbers {
    a: f64,
    b: f64,
}

#[derive(Serialize)]
struct Result {
    result: f64,
    operation: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn divide(numbers: web::Json<Numbers>) -> impl Responder {
    if numbers.b == 0.0 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Division by zero is not allowed".to_string(),
        });
    }
    
    let result = numbers.a / numbers.b;
    println!("Dividing {} / {} = {}", numbers.a, numbers.b, result);
    
    HttpResponse::Ok().json(Result {
        result,
        operation: format!("{} / {}", numbers.a, numbers.b),
    })
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Division service is healthy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Division service on port 8004");
    
    HttpServer::new(|| {
        App::new()
            .route("/divide", web::post().to(divide))
            .route("/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:8004")?
    .run()
    .await
}
