use crate::db::Database;
use crate::models::{Care, CreateCare};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

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

/// Handler to add a new care
pub async fn add_care(
    State(db): State<Database>,
    Json(payload): Json<CreateCare>,
) -> Result<(StatusCode, Json<Care>), (StatusCode, String)> {
    // Validate required fields
    if payload.type_of_care.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Type of care is required and cannot be empty".to_string(),
        ));
    }
    if payload.frequency.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Frequency is required and cannot be empty".to_string(),
        ));
    }

    let mut client = db.connect().await.map_err(|e| {
        eprintln!("Database connection error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database connection error: {}", e),
        )
    })?;

    // Determine next id manually
    let id_query = "SELECT ISNULL(MAX(cares_id),0)+1 AS next_id FROM Cares";
    let id_stream = client.query(id_query, &[]).await.map_err(|e| {
        eprintln!("ID query error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("ID query error: {}", e),
        )
    })?;
    let id_rows = id_stream.into_first_result().await.map_err(|e| {
        eprintln!("ID result error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("ID result error: {}", e),
        )
    })?;
    let new_id = id_rows.first().and_then(|r| r.get::<i32, _>(0)).ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Failed to compute next id".to_string(),
    ))?;

    let insert_query = r#"
        INSERT INTO Cares (cares_id, type_of_care, description, frequency)
        VALUES (@P1, @P2, @P3, @P4)
    "#;

    client
        .execute(
            insert_query,
            &[
                &new_id,
                &payload.type_of_care,
                &payload.description,
                &payload.frequency,
            ],
        )
        .await
        .map_err(|e| {
            eprintln!("Insert error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Insert error: {}", e),
            )
        })?;

    let created = Care {
        cares_id: new_id,
        type_of_care: payload.type_of_care,
        description: payload.description,
        frequency: payload.frequency,
    };

    Ok((StatusCode::CREATED, Json(created)))
}
