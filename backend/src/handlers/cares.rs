use crate::db::Database;
use crate::models::{Care, CreateCare};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::NaiveDate;

pub async fn get_cares(
    State(db): State<Database>,
) -> Result<Json<Vec<Care>>, (StatusCode, String)> {
    let mut client = db.connect().await.map_err(|e| {
        eprintln!("Database connection error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database connection error: {}", e),
        )
    })?;

    let query =
        "SELECT type_of_care, description, frequency, cares_id FROM Cares ORDER BY cares_id";

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

    let care: Vec<Care> = rows
        .iter()
        .map(|row| Care {
            type_of_care: row.get::<&str, _>(0).unwrap_or("").to_string(),
            description: row.get::<&str, _>(1).map(|s| s.to_string()),
            frequency: row.get::<&str, _>(2).unwrap_or("").to_string(),
            cares_id: row.get::<i32, _>(3).unwrap_or(0),
        })
        .collect();

    Ok(Json(care))
}

pub async fn get_care_by_id(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Result<Json<Care>, (StatusCode, String)> {
    let mut client = db.connect().await.map_err(|e| {
        eprintln!("Database connection error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database connection error: {}", e),
        )
    })?;

    let query =
        "SELECT type_of_care, description, frequency, cares_id FROM Cares WHERE cares_id = @P1";

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
        let care = Care {
            type_of_care: row.get::<&str, _>(0).unwrap_or("").to_string(),
            description: row.get::<&str, _>(1).map(|s| s.to_string()),
            frequency: row.get::<&str, _>(2).unwrap_or("").to_string(),
            cares_id: row.get::<i32, _>(3).unwrap_or(0),
        };
        Ok(Json(care))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            format!("Animal care with id {} not found", id),
        ))
    }
}
