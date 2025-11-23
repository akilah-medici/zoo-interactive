use crate::db::Database;
use crate::models::{AnimalCare, CreateAnimalCare};
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

pub async fn get_animal_care_by_id(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Result<Json<AnimalCare>, (StatusCode, String)> {
    let mut client = db.connect().await.map_err(|e| {
        eprintln!("Database connection error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database connection error: {}", e),
        )
    })?;

    let query = "SELECT date_of_care, fk_Cares_cares_id, fk_Animal_animal_id, animal_care_id FROM Animal_Care_have WHERE animal_care_id = @P1";

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
        let animal_care = AnimalCare {
            date_of_care: row.get(0),
            fk_cares_cares_id: row.get::<i32, _>(1).unwrap_or(0),
            fk_animal_animal_id: row.get::<i32, _>(2).unwrap_or(0),
            animal_care_id: row.get::<i32, _>(3).unwrap_or(0),
        };
        Ok(Json(animal_care))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            format!("Animal care with id {} not found", id),
        ))
    }
}

pub async fn get_animal_care_by_animal_id(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Result<Json<AnimalCare>, (StatusCode, String)> {
    let mut client = db.connect().await.map_err(|e| {
        eprintln!("Database connection error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database connection error: {}", e),
        )
    })?;

    let query = "SELECT date_of_care, fk_Cares_cares_id, fk_Animal_animal_id, animal_care_id FROM Animal_Care_have WHERE fk_Animal_animal_id = @P1";

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
        let animal_care = AnimalCare {
            date_of_care: row.get(0),
            fk_cares_cares_id: row.get::<i32, _>(1).unwrap_or(0),
            fk_animal_animal_id: row.get::<i32, _>(2).unwrap_or(0),
            animal_care_id: row.get::<i32, _>(3).unwrap_or(0),
        };
        Ok(Json(animal_care))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            format!("Animal care with id {} not found", id),
        ))
    }
}

pub async fn add_animal_care(
    State(db): State<Database>,
    Json(payload): Json<CreateAnimalCare>,
) -> Result<(StatusCode, Json<AnimalCare>), (StatusCode, String)> {
    let mut client = db.connect().await.map_err(|e| {
        eprintln!("Database connection error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database connection error: {}", e),
        )
    })?;

    // Parse date
    let parsed_date: Option<NaiveDate> = if let Some(d) = &payload.date_of_care {
        if d.contains('/') {
            NaiveDate::parse_from_str(d, "%d/%m/%Y").ok()
        } else {
            NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()
        }
    } else {
        None
    };

    let id_query = "SELECT ISNULL(MAX(animal_care_id),0)+1 AS next_id FROM Animal_Care_have";
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
        INSERT INTO Animal_Care_have (animal_care_id, date_of_care, fk_Cares_cares_id, fk_Animal_animal_id)
        VALUES (@P1, @P2, @P3, @P4)
    "#;

    client
        .execute(
            insert_query,
            &[
                &new_id,
                &parsed_date,
                &payload.fk_cares_cares_id,
                &payload.fk_animal_animal_id,
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

    let created = AnimalCare {
        animal_care_id: new_id,
        date_of_care: parsed_date,
        fk_cares_cares_id: payload.fk_cares_cares_id,
        fk_animal_animal_id: payload.fk_animal_animal_id,
    };

    Ok((StatusCode::CREATED, Json(created)))
}
