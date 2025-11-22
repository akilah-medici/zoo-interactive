use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Model for the Animal_Care_have table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimalCare {
    pub animal_care_id: i32,
    pub date_of_care: Option<NaiveDate>,
    pub fk_cares_cares_id: i32,
    pub fk_animal_animal_id: i32,
}
pub struct CreateAnimalCare {
    pub date_of_care: Option<NaiveDate>,
    pub fk_cares_cares_id: i32,
    pub fk_animal_animal_id: i32,
}
pub struct UpdateAnimalCare {
    pub date_of_care: Option<NaiveDate>,
    pub fk_cares_cares_id: i32,
    pub fk_animal_animal_id: i32,
}
