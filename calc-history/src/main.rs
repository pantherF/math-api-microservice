use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

type DbPool = Pool<Postgres>;

#[derive(Serialize, Deserialize)]
struct CalculationResult {
    result: f64,
    operation: String,
}

async fn record_calculation(
    calculation: web::Json<CalculationResult>, 
    db_pool: web::Data<DbPool>
) -> impl Responder {
    match sqlx::query(
        "INSERT INTO calculations (operation, result) VALUES ($1, $2)"
    )
    .bind(&calculation.operation)
    .bind(calculation.result)
    .execute(db_pool.get_ref())
    .await {
        Ok(_) => {
            println!("Recorded calculation: {}", calculation.operation);
            HttpResponse::Ok().body("Calculation recorded")
        },
        Err(e) => {
            println!("Failed to record calculation: {}", e);
            HttpResponse::InternalServerError().body("Failed to record calculation")
        }
    }
}

async fn get_history(db_pool: web::Data<DbPool>) -> impl Responder {
    match sqlx::query_as!(
        CalculationResult,
        "SELECT operation, result FROM calculations ORDER BY created_at DESC LIMIT 100"
    )
    .fetch_all(db_pool.get_ref())
    .await {
        Ok(calculations) => {
            HttpResponse::Ok().json(calculations)
        },
        Err(e) => {
            println!("Failed to retrieve calculations: {}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve calculation history")
        }
    }
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("History service is healthy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting History service on port 8005");
    
    // Get database URL from environment or use default
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@db:5432/calculator".to_string());
    
    // Set up the database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");
    
    // Run the database migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/record", web::post().to(record_calculation))
            .route("/history", web::get().to(get_history))
            .route("/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:8005")?
    .run()
    .await
}
