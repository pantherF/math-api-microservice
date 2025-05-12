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

async fn subtract(numbers: web::Json<Numbers>) -> impl Responder {
    let result = numbers.a - numbers.b;
    println!("Subtracting {} - {} = {}", numbers.a, numbers.b, result);
    
    HttpResponse::Ok().json(Result {
        result,
        operation: format!("{} - {}", numbers.a, numbers.b),
    })
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Subtraction service is healthy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Subtraction service on port 8002");
    
    HttpServer::new(|| {
        App::new()
            .route("/subtract", web::post().to(subtract))
            .route("/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:8002")?
    .run()
    .await
}
