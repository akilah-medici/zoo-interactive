use axum::routing::{Router, get, post, put, delete};
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

    let cors = CorsLayer::permissive();

    let animals_router = Router::new()
        .route("/list", get(get_animals))
        .route("/add", post(add_animal))
        .route("/animals/{id}", get(get_animal_by_id))
        .route("/deactivate/{id}", post(deactivate_animal))
        .route("/update/{id}", put(update_animal))
        .route("/delete/{id}", delete(delete_animal));

    let cares_router = Router::new()
        .route("/list", get(get_cares))
        .route("/by-id/{id}", get(get_care_by_id))
        .route("/add", post(add_care))
        .route("/update/{id}", put(update_care))
        .route("/delete/{id}", delete(delete_care));

    let animal_cares_router = Router::new()
        .route("/list", get(get_animal_cares))
        .route("/by-id/{id}", get(get_animal_care_by_id))
        .route("/by-animal/by-id/{id}", get(get_animal_care_by_animal_id))
        .route("/add", post(add_animal_care));

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
    println!("  GET    /message                         - Test endpoint");
    println!("  GET    /animals/list                    - List all animals (active)");
    println!("  GET    /animals/animals/id              - Get animal by ID");
    println!("  POST   /animals/add                     - Add new animal");
    println!("  PUT    /animals/update/id               - Update animal");
    println!("  POST   /animals/deactivate/id           - Soft delete animal");
    println!("  DELETE /animals/delete/id               - Hard delete animal");
    println!("  GET    /cares/list                      - List all cares");
    println!("  GET    /cares/by-id/id                  - Get care by ID");
    println!("  POST   /cares/add                       - Add new care");
    println!("  PUT    /cares/update/id                 - Update care");
    println!("  DELETE /cares/delete/id                 - Delete care");
    println!("  GET    /animal-cares/list               - List all animal-care relations");
    println!("  GET    /animal-cares/by-id/id           - Get animal-care relation by ID");
    println!("  GET    /animal-cares/by-animal/by-id/id - Get all cares for animal by animal ID");
    println!("  POST   /animal-cares/add                - Add animal-care relation");

    axum::serve(listener, app).await.unwrap();
}
