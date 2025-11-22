use crate::db::Database;
use crate::models::{Animal, CreateAnimal, UpdateAnimal};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::NaiveDate;

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

    let query = "SELECT animal_id, name, specie, habitat, description, country_of_origin, date_of_birth FROM Animal WHERE is_active = 1 ORDER BY animal_id";

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

    let query = "SELECT animal_id, name, specie, habitat, description, country_of_origin, date_of_birth FROM Animal WHERE animal_id = @P1 AND is_active = 1";

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

/// Handler to insert a new animal into the database

pub async fn add_animal(
    State(db): State<Database>,
    Json(payload): Json<CreateAnimal>,
) -> Result<(StatusCode, Json<Animal>), (StatusCode, String)> {
    // Validate required fields
    if payload.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Name is required and cannot be empty".to_string(),
        ));
    }
    if payload.specie.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Specie is required and cannot be empty".to_string(),
        ));
    }

    let mut client = db.connect().await.map_err(|e| {
        eprintln!("Database connection error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database connection error: {}", e),
        )
    })?;

    // Parse date_of_birth if provided (supporting dd/MM/yyyy and yyyy-MM-dd)
    let parsed_date: Option<NaiveDate> = if let Some(d) = &payload.date_of_birth {
        if d.contains('/') {
            NaiveDate::parse_from_str(d, "%d/%m/%Y").ok()
        } else {
            NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()
        }
    } else {
        None
    };

    // Determine next id manually (table lacks IDENTITY)
    let id_query = "SELECT ISNULL(MAX(animal_id),0)+1 AS next_id FROM Animal";
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
        INSERT INTO Animal (animal_id, name, specie, habitat, description, country_of_origin, date_of_birth, is_active)
        VALUES (@P1, @P2, @P3, @P4, @P5, @P6, @P7, 1)
    "#;

    client
        .execute(
            insert_query,
            &[
                &new_id,                    // @P1
                &payload.name,              // @P2
                &payload.specie,            // @P3
                &payload.habitat,           // @P4
                &payload.description,       // @P5
                &payload.country_of_origin, // @P6
                &parsed_date,               // @P7
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

    let created = Animal {
        animal_id: new_id,
        name: payload.name,
        specie: payload.specie,
        habitat: payload.habitat,
        description: payload.description,
        country_of_origin: payload.country_of_origin,
        date_of_birth: parsed_date,
    };

    Ok((StatusCode::CREATED, Json(created)))
}

/// Handler to soft-delete an animal by setting is_active to 0
pub async fn deactivate_animal(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut client = db.connect().await.map_err(|e| {
        eprintln!("Database connection error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database connection error: {}", e),
        )
    })?;

    let query = "UPDATE Animal SET is_active = 0 WHERE animal_id = @P1 AND is_active = 1";

    let rows_affected = client.execute(query, &[&id]).await.map_err(|e| {
        eprintln!("Update error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Update error: {}", e),
        )
    })?;

    if rows_affected.total() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Animal with id {} not found or already inactive", id),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Handler to update an existing animal
pub async fn update_animal(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateAnimal>,
) -> Result<Json<Animal>, (StatusCode, String)> {
    // Validate required fields if provided
    if let Some(ref name) = payload.name {
        if name.trim().is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Name cannot be empty".to_string(),
            ));
        }
    }
    if let Some(ref specie) = payload.specie {
        if specie.trim().is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Specie cannot be empty".to_string(),
            ));
        }
    }

    let mut client = db.connect().await.map_err(|e| {
        eprintln!("Database connection error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database connection error: {}", e),
        )
    })?;

    // Parse date_of_birth if provided
    let parsed_date: Option<NaiveDate> = if let Some(d) = &payload.date_of_birth {
        if d.contains('/') {
            NaiveDate::parse_from_str(d, "%d/%m/%Y").ok()
        } else {
            NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()
        }
    } else {
        None
    };

    // Build dynamic UPDATE query based on provided fields
    let update_query = r#"
        UPDATE Animal 
        SET name = COALESCE(@P2, name),
            specie = COALESCE(@P3, specie),
            habitat = COALESCE(@P4, habitat),
            description = COALESCE(@P5, description),
            country_of_origin = COALESCE(@P6, country_of_origin),
            date_of_birth = COALESCE(@P7, date_of_birth)
        WHERE animal_id = @P1 AND is_active = 1
    "#;

    let rows_affected = client
        .execute(
            update_query,
            &[
                &id,
                &payload.name,
                &payload.specie,
                &payload.habitat,
                &payload.description,
                &payload.country_of_origin,
                &parsed_date,
            ],
        )
        .await
        .map_err(|e| {
            eprintln!("Update error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Update error: {}", e),
            )
        })?;

    if rows_affected.total() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Animal with id {} not found or inactive", id),
        ));
    }

    // Fetch and return the updated animal
    let query = "SELECT animal_id, name, specie, habitat, description, country_of_origin, date_of_birth FROM Animal WHERE animal_id = @P1 AND is_active = 1";
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
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch updated animal".to_string(),
        ))
    }
}

/// Original handler - kept for backward compatibility
pub async fn initial_page() -> &'static str {
    "Hello from backend!"
}
