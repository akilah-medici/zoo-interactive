use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Care {
    pub cares_id: i32,
    pub type_of_care: String,
    pub frequency: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCare {
    pub type_of_care: String,
    pub frequency: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCrare {
    pub type_of_care: String,
    pub frequency: String,
    pub description: Option<String>,
}
