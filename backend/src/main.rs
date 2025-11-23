use axum::routing::{Router, get, post, put};
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

    let animals_router = Router::new()
        .route("/list", get(get_animals))
        .route("/add", post(add_animal))
        .route("/animals/{id}", get(get_animal_by_id))
        .route("/deactivate/{id}", post(deactivate_animal))
        .route("/update/{id}", put(update_animal));

    let cares_router = Router::new()
        .route("/list", get(get_cares))
        .route("/by-id/{id}", get(get_care_by_id))
        .route("/add", post(add_care));

    let animal_cares_router = Router::new()
        .route("/list", get(get_animal_cares))
        .route("/by-id/{id}", get(get_animal_care_by_id))
        .route("/by-animal/by-id/{id}", get(get_animal_care_by_animal_id))
        .route("/add", post(add_animal_care));

    // Build application routes
    let app = Router::new()
        .route("/message", get(initial_page))
        .nest("/animals", animals_router)
        .nest("/cares", cares_router)
        .nest("/animal-cares", animal_cares_router)
        .with_state(database)
        .layer(cors);

    println!("Binding to 0.0.0.0:3000...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server listening on port 3000");
    println!("Available endpoints:");
    println!("  GET /message - Test endpoint");
    println!("  GET /animals/list - Get all animals");
    println!("  GET /cares/list - Get all cares for animals");
    //println!("  GET /animals/:id - Get animal by ID");
    println!("  POST /animals/add - Create new animal");

    axum::serve(listener, app).await.unwrap();
}
