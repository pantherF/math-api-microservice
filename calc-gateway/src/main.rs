use actix_web::{web, App, HttpServer, Responder, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use awc::Client;

#[derive(Deserialize, Serialize)]  // Added Serialize here
struct Numbers {
    a: f64,
    b: f64,
}

#[derive(Serialize, Deserialize)]
struct CalculationResult {
    result: f64,
    operation: String,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

async fn forward_to_service(numbers: web::Json<Numbers>, url: &str) -> Result<HttpResponse, Error> {
    let client = Client::default();
    
    let mut response = client
        .post(url)
        .send_json(&numbers)
        .await
        .map_err(|e| {
            println!("Failed to forward request: {}", e);
            actix_web::error::ErrorInternalServerError("Service unavailable")
        })?;
    
    if response.status().is_success() {
        let result = response
            .json::<CalculationResult>()
            .await
            .map_err(|e| {
                println!("Failed to parse response: {}", e);
                actix_web::error::ErrorInternalServerError("Failed to parse service response")
            })?;
            
        // Send calculation to history service
        let _ = client
            .post("http://history:8005/record")
            .send_json(&result)
            .await;
            
        Ok(HttpResponse::Ok().json(result))
    } else {
        let error = response
            .json::<ErrorResponse>()
            .await
            .map_err(|e| {
                println!("Failed to parse error response: {}", e);
                actix_web::error::ErrorInternalServerError("Failed to parse service error")
            })?;
            
        Ok(HttpResponse::BadRequest().json(error))
    }
}

async fn add(numbers: web::Json<Numbers>) -> Result<HttpResponse, Error> {
    forward_to_service(numbers, "http://add:8001/add").await
}

async fn subtract(numbers: web::Json<Numbers>) -> Result<HttpResponse, Error> {
    forward_to_service(numbers, "http://subtract:8002/subtract").await
}

async fn multiply(numbers: web::Json<Numbers>) -> Result<HttpResponse, Error> {
    forward_to_service(numbers, "http://multiply:8003/multiply").await
}

async fn divide(numbers: web::Json<Numbers>) -> Result<HttpResponse, Error> {
    forward_to_service(numbers, "http://divide:8004/divide").await
}

async fn get_history() -> Result<HttpResponse, Error> {
    let client = Client::default();
    
    let mut response = client  // Added 'mut' here
        .get("http://history:8005/history")
        .send()
        .await
        .map_err(|e| {
            println!("Failed to retrieve history: {}", e);
            actix_web::error::ErrorInternalServerError("History service unavailable")
        })?;
    
    if !response.status().is_success() {
        return Ok(HttpResponse::InternalServerError().body("Failed to retrieve calculation history"));
    }
    
    let history: Vec<CalculationResult> = response
        .json()
        .await
        .map_err(|e| {
            println!("Failed to parse history response: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to parse history response")
        })?;
    
    Ok(HttpResponse::Ok().json(history))
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("API Gateway is healthy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting API Gateway on port 8000");
    
    HttpServer::new(|| {
        App::new()
            .route("/calculate/add", web::post().to(add))
            .route("/calculate/subtract", web::post().to(subtract))
            .route("/calculate/multiply", web::post().to(multiply))
            .route("/calculate/divide", web::post().to(divide))
            .route("/history", web::get().to(get_history))
            .route("/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
