use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Animal model representing the animals table in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animal {
    pub animal_id: i32,
    pub name: String,
    pub specie: String,
    pub habitat: Option<String>,
    pub description: Option<String>,
    pub country_of_origin: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
}
/// DTO for creating a new animal
#[derive(Debug, Deserialize)]
pub struct CreateAnimal {
    pub name: String,
    pub specie: String,
    pub habitat: Option<String>,
    pub description: Option<String>,
    pub country_of_origin: Option<String>,
    pub date_of_birth: Option<String>,
}

/// DTO for updating an existing animal
#[derive(Debug, Deserialize)]
pub struct UpdateAnimal {
    pub name: Option<String>,
    pub specie: Option<String>,
    pub habitat: Option<String>,
    pub description: Option<String>,
    pub country_of_origin: Option<String>,
    pub date_of_birth: Option<String>,
}
