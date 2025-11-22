use crate::db::Database;
use crate::models::AnimalCare;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::NaiveDate;

pub async fn get_animal_cares(
    State(db): State<Database>,
) -> Result<Json<Vec<AnimalCare>>, (StatusCode, String)> {
    let mut client = db.connect().await.map_err(|e| {
        eprintln!("Database connection error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database connection error: {}", e),
        )
    })?;

    let query = "SELECT date_of_care, fk_Cares_cares_id, fk_Animal_animal_id, animal_care_id FROM Animal_Care_have ORDER BY animal_care_id";

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

    let animal_cares: Vec<AnimalCare> = rows
        .iter()
        .map(|row| AnimalCare {
            date_of_care: row.get(0),
            fk_cares_cares_id: row.get::<i32, _>(1).unwrap_or(0),
            fk_animal_animal_id: row.get::<i32, _>(2).unwrap_or(0),
            animal_care_id: row.get::<i32, _>(3).unwrap_or(0),
        })
        .collect();

    Ok(Json(animal_cares))
}
