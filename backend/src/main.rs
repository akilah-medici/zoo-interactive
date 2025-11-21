use axum::routing::{Router, get};
use tower_http::cors::CorsLayer;

pub mod db;
pub mod handlers;
pub mod models;

use crate::db::Database;
use crate::handlers::*;

#[tokio::main]
async fn main() {
    println!("Starting backend server...");

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize database configuration
    let database = Database::new().expect("Failed to create database configuration");

    // Test database connection
    database
        .test_connection()
        .await
        .expect("Failed to connect to database");

    // Configure CORS
    let cors = CorsLayer::permissive();

    // Build application routes
    let app = Router::new()
        .route("/message", get(initial_page))
        .route("/animals", get(get_animals))
        .route("/animals/{id}", get(get_animal_by_id))
        .with_state(database)
        .layer(cors);

    println!("Binding to 0.0.0.0:3000...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server listening on port 3000");
    println!("Available endpoints:");
    println!("  GET /message - Test endpoint");
    println!("  GET /animals - Get all animals");
    println!("  GET /animals/:id - Get animal by ID");

    axum::serve(listener, app).await.unwrap();
}
