use crate::db::Database;
use crate::models::Animal;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

/// Handler to get all animals from the database
pub async fn get_animals(
    State(db): State<Database>,
) -> Result<Json<Vec<Animal>>, (StatusCode, String)> {
    let mut client = db.connect().await.map_err(|e| {
        eprintln!("Database connection error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database connection error: {}", e),
        )
    })?;

    let query = "SELECT animal_id, name, specie, habitat, description, country_of_origin, date_of_birth FROM Animal ORDER BY animal_id";

    let stream = client.query(query, &[]).await.map_err(|e| {
        eprintln!("Query error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Query error: {}", e),
        )
    })?;

    let rows = stream.into_first_result().await.map_err(|e| {
        eprintln!("Result error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Result error: {}", e),
        )
    })?;

    let animals: Vec<Animal> = rows
        .iter()
        .map(|row| Animal {
            animal_id: row.get::<i32, _>(0).unwrap_or(0),
            name: row.get::<&str, _>(1).unwrap_or("").to_string(),
            specie: row.get::<&str, _>(2).unwrap_or("").to_string(),
            habitat: row.get::<&str, _>(3).map(|s| s.to_string()),
            description: row.get::<&str, _>(4).map(|s| s.to_string()),
            country_of_origin: row.get::<&str, _>(5).map(|s| s.to_string()),
            date_of_birth: row.get(6),
        })
        .collect();

    Ok(Json(animals))
}

/// Handler to get a specific animal by ID
pub async fn get_animal_by_id(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Result<Json<Animal>, (StatusCode, String)> {
    let mut client = db.connect().await.map_err(|e| {
        eprintln!("Database connection error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database connection error: {}", e),
        )
    })?;

    let query = "SELECT animal_id, name, specie, habitat, description, country_of_origin, date_of_birth FROM Animal WHERE animal_id = @P1";

    let stream = client.query(query, &[&id]).await.map_err(|e| {
        eprintln!("Query error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Query error: {}", e),
        )
    })?;

    let rows = stream.into_first_result().await.map_err(|e| {
        eprintln!("Result error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Result error: {}", e),
        )
    })?;

    if let Some(row) = rows.first() {
        let animal = Animal {
            animal_id: row.get::<i32, _>(0).unwrap_or(0),
            name: row.get::<&str, _>(1).unwrap_or("").to_string(),
            specie: row.get::<&str, _>(2).unwrap_or("").to_string(),
            habitat: row.get::<&str, _>(3).map(|s| s.to_string()),
            description: row.get::<&str, _>(4).map(|s| s.to_string()),
            country_of_origin: row.get::<&str, _>(5).map(|s| s.to_string()),
            date_of_birth: row.get(6),
        };
        Ok(Json(animal))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            format!("Animal with id {} not found", id),
        ))
    }
}

/// Original handler - kept for backward compatibility
pub async fn initial_page() -> &'static str {
    println!("peido 3");
    "Hello from backend!"
}
